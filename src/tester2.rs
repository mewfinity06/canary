#![allow(dead_code, unused_imports)]

use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Read},
    path::{Path, PathBuf},
};

use anyhow::bail;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use lexer::{
    Lexer,
    token::{self, TokenType},
};
use utils::{error, info};

const USE_README_TESTS: bool = true;

#[derive(Deserialize, Debug)]
pub struct Test {
    file: String,
    expected: Vec<String>,
}

impl Test {
    pub fn empty() -> Self {
        Self {
            file: String::new(),
            expected: Vec::new(),
        }
    }
}

impl Serialize for Test {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Test", 2)?;
        state.serialize_field("file", &self.file)?;
        state.serialize_field("expected", &self.expected)?;
        state.end()
    }
}

#[derive(Deserialize, Debug)]
pub struct TestList {
    tests: Vec<Test>,
}

impl TestList {
    pub fn empty() -> Self {
        Self { tests: Vec::new() }
    }
}

impl Serialize for TestList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("TestList", 1)?;
        state.serialize_field("tests", &self.tests)?;
        state.end()
    }
}

pub fn build_tests() -> anyhow::Result<()> {
    let test_dir = Path::new("./tests/");
    let expected_path = test_dir.join("expected.json");

    let paths_to_process: Vec<PathBuf> = fs::read_dir(test_dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file() && *path != expected_path)
        .collect();

    if !USE_README_TESTS {
        info!("Building {} tests", paths_to_process.len());
    } else {
        info!("Building {} tests", paths_to_process.len() + 1);
    }

    let collected_tests: Vec<Test> = paths_to_process
        .into_par_iter()
        .map(|path| {
            info!("Building {}", path.to_str().unwrap());
            let content = fs::read_to_string(&path)?;
            let mut test = Test::empty();
            test.file = path.to_str().unwrap_or("").to_string();

            let lexer = Lexer::new(&content);
            test.expected = lexer
                .into_iter()
                .map(|token| match token.kind {
                    TokenType::Error(e) => {
                        panic!("{e}");
                    }
                    _ => token.kind.into_str().to_string(),
                })
                .collect();

            Ok(test)
        })
        .collect::<anyhow::Result<Vec<Test>>>()?;

    if USE_README_TESTS {
        build_readme_test()?;
    }

    let tests = TestList {
        tests: collected_tests,
    };

    let expected_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&expected_path)?;
    let mut writer = BufWriter::new(expected_file);

    serde_json::to_writer_pretty(&mut writer, &tests)?;

    Ok(())
}

pub fn run_tests() -> anyhow::Result<()> {
    let expected_file = File::open("./tests/expected.json")?;
    let tests: TestList = serde_json::from_reader(expected_file)?;

    let mut passed: usize = 0;
    let mut total = tests.tests.len();

    'tests: for test in &tests.tests {
        let test_file = File::open(&test.file)?;
        let mut content = String::new();
        BufReader::new(test_file).read_to_string(&mut content)?;

        let lexer = Lexer::new(&content);
        let mut tokens = lexer.into_iter();

        for expected_str in &test.expected {
            match tokens.next() {
                Some(actual_token) => {
                    let actual_str = actual_token.kind.into_str();
                    if actual_str != expected_str.as_str() {
                        error!(
                            "{}:{}:{}: Token mismatch\n\tExpected: {}\n\t     Got: {}",
                            test.file,
                            actual_token.loc.line,
                            actual_token.loc.col,
                            expected_str,
                            actual_str
                        );
                        continue 'tests;
                    }
                }
                None => {
                    error!("{}: Unexpected EoI", test.file);
                    continue 'tests;
                }
            }
        }

        if let Some(extra_token) = tokens.next() {
            error!(
                "{}:{}:{}: Unexpected extra token found: {}",
                test.file,
                extra_token.loc.line,
                extra_token.loc.col,
                extra_token.kind.into_str()
            );
            continue 'tests;
        }

        passed += 1;
        info!("{} passed", test.file);
    }

    if USE_README_TESTS {
        total += 1;
        match run_readme_test() {
            Ok(_) => passed += 1,
            Err(e) => {
                error!("{e}");
            }
        }
    }

    let percent_passed: usize = ((passed as f64) / (total as f64) * 100.0) as usize;
    info!("{}/{} tests passed ({}%)", passed, total, percent_passed);

    Ok(())
}

pub fn build_and_run_tests() -> anyhow::Result<()> {
    println!("-------------------------------------------");
    build_tests()?;
    println!("-------------------------------------------");
    run_tests()?;
    println!("-------------------------------------------");
    Ok(())
}

const README_TEST_MAGIC: &str = "```canary test ";
const END_README_MAGIC: &str = "```";

pub fn build_readme_test() -> anyhow::Result<()> {
    let readme_test = Path::new("./tests/readme_test.json");
    let mut test = Test::empty();

    let readme = File::open("./README.md")?;
    let reader = BufReader::new(readme);

    let mut test_str = String::new();
    let mut test_name = String::new();
    let mut in_test = false;

    for line in reader.lines() {
        let line = line?;
        if line.starts_with(README_TEST_MAGIC) {
            in_test = true;
            test_name.push(':');
            test_name.push_str(line.strip_prefix(README_TEST_MAGIC).unwrap());
            continue;
        } else if line.starts_with(END_README_MAGIC) {
            in_test = false;
            let lexer = Lexer::new(&test_str);
            test.file = readme_test.to_str().unwrap().to_string();
            test.file.push_str(&test_name);
            test.expected = lexer
                .into_iter()
                .map(|token| token.kind.into_str().to_string())
                .collect();
        }

        if in_test {
            test_str.push_str(line.as_str());
            test_str.push('\n');
        }
    }

    let expected_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&readme_test)?;
    let mut writer = BufWriter::new(expected_file);

    serde_json::to_writer_pretty(&mut writer, &test)?;

    Ok(())
}

pub fn run_readme_test() -> anyhow::Result<()> {
    let readme_test_file = File::open("./tests/readme_test.json")?;
    let readme_test: Test = serde_json::from_reader(readme_test_file)?;

    let readme = File::open("./README.md")?;
    let reader = BufReader::new(readme);

    let mut test_str = String::new();
    let mut in_test = false;

    let mut tokens = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with(README_TEST_MAGIC) {
            in_test = true;
            continue;
        } else if line.starts_with(END_README_MAGIC) {
            in_test = false;
            let lexer = Lexer::new(&test_str);
            tokens = lexer.collect();
        }

        if in_test {
            test_str.push_str(line.as_str());
            test_str.push('\n');
        }
    }

    if tokens.len() != readme_test.expected.len() {
        bail!(
            "Expected {} tokens but got {} tokens",
            tokens.len(),
            readme_test.expected.len()
        );
    }

    let mut expected_iter = readme_test.expected.iter();
    for token in tokens {
        let expected = expected_iter.next().unwrap();
        let actual = token.kind.into_str();
        if actual != expected {
            bail!("Expected {} but got {}", expected, actual);
        }
    }

    info!("{} passed", readme_test.file);

    Ok(())
}
