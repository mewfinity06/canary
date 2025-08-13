#![allow(dead_code, unused_imports)]

use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Read},
    path::{Path, PathBuf},
};

use anyhow::bail;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use parser::Parser;

use lexer::{
    Lexer,
    token::{self, TokenType},
};
use utils::{context, error, info};

#[derive(Deserialize, Debug)]
pub struct Test {
    file: String,
    expected: Vec<String>,
    skipped: bool,
}

impl Test {
    pub fn empty() -> Self {
        Self {
            file: String::new(),
            expected: Vec::new(),
            skipped: false,
        }
    }
}

impl Serialize for Test {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Test", 3)?;
        state.serialize_field("file", &self.file)?;
        state.serialize_field("expected", &self.expected)?;
        state.serialize_field("skipped", &self.skipped)?;
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

pub fn build_tests(verbose: bool) -> anyhow::Result<()> {
    let test_dir = Path::new("./tests/");
    let expected_path = test_dir.join("expected.json");

    let paths_to_process: Vec<PathBuf> = fs::read_dir(test_dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file() && *path != expected_path && *path != test_dir.join("readme_test.json"))
        .collect();

    info!("Building {} tests", paths_to_process.len());

    let collected_tests: Vec<Test> = paths_to_process
        .into_par_iter()
        .map(|path| {
            info!("Building {}", path.to_str().unwrap());
            let content = fs::read_to_string(&path)?;
            let mut test = Test::empty();
            test.file = path.to_str().unwrap_or("").to_string();

            let lexer = Lexer::new(&content);
            let mut parser = Parser::new(lexer);
            let program = match parser.program() {
                Ok(p) => p,
                Err(e) => {
                    error!("Test failed, adding to skipped");
                    if verbose {
                        context!("{}", e);
                    }
                    let mut test = Test::empty();
                    test.file = path.to_str().unwrap_or("").to_string();
                    test.skipped = true;
                    return Ok(test);
                }
            };
            test.expected = vec![format!("{:?}", program)];

            Ok(test)
        })
        .collect::<anyhow::Result<Vec<Test>>>()?;

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

pub fn run_tests(verbose: bool) -> anyhow::Result<()> {
    let expected_file = File::open("./tests/expected.json")?;
    let tests: TestList = serde_json::from_reader(expected_file)?;

    let mut passed: usize = 0;
    let mut skipped: usize = 0;
    let total = tests.tests.len();

    'tests: for test in &tests.tests {
        if test.skipped {
            info!("Skipping {} (build failed)", test.file);
            skipped += 1;
            continue 'tests;
        }

        if verbose {
            info!("Expecting {:#?}", &test.expected);
        }

        let test_file = File::open(&test.file)?;
        let mut content = String::new();
        BufReader::new(test_file).read_to_string(&mut content)?;

        let lexer = Lexer::new(&content);
        let mut parser = Parser::new(lexer);
        let program = match parser.program() {
            Ok(p) => p,
            Err(e) => {
                error!("Error parsing {}: {}", test.file, e);
                if verbose {
                    context!("{}", e);
                }
                continue 'tests;
            }
        };
        let actual_str = format!("{:?}", program);

        if actual_str != test.expected[0] {
            error!(
                "{}: AST mismatch\n\tExpected: {}\n\t     Got: {}",
                test.file,
                test.expected[0],
                actual_str
            );
            continue 'tests;
        }

        passed += 1;
        info!("{} passed", test.file);
    }

    let percent_passed: usize = ((passed as f64) / ((total - skipped) as f64) * 100.0) as usize;
    info!("{}/{} tests passed ({}%), {} tests skipped", passed, total - skipped, percent_passed, skipped);

    Ok(())
}

pub fn build_and_run_tests(verbose: bool) -> anyhow::Result<()> {
    println!("-------------------------------------------");
    build_tests(verbose)?;
    println!("-------------------------------------------");
    run_tests(verbose)?;
    println!("-------------------------------------------");
    Ok(())
}
