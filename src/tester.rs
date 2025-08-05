#![allow(dead_code)]

use std::{
    fs::{self, File, OpenOptions},
    io::{BufReader, BufWriter, Read},
    path::{Path, PathBuf},
};

use anyhow::Result;
use rayon::prelude::*;
use serde::Deserialize;

use utils::{error, info};
use lexer::Lexer;

#[derive(Deserialize, Debug)]
struct Tests {
    tests: Vec<Test>,
}

impl Tests {
    fn empty() -> Self {
        Self { tests: Vec::new() }
    }
}

#[derive(Deserialize, Debug)]
struct Test {
    file: String,
    expected: Vec<String>,
}

impl Test {
    fn empty() -> Self {
        Self {
            file: String::new(),
            expected: Vec::new(),
        }
    }
}

pub fn test_compiler() -> anyhow::Result<()> {
    let expected_file = File::open("./tests/expected.json")?;
    let tests: Tests = serde_json::from_reader(expected_file)?;

    let mut passed: usize = 0;
    let total = tests.tests.len();

    let longest_name = tests.tests.iter().map(|t| t.file.len()).max().unwrap_or(0);

    println!("----------------------------------");
    'tests: for test in &tests.tests {
        let test_file = File::open(&test.file)?;
        let mut content = String::new();
        BufReader::new(test_file).read_to_string(&mut content)?;

        let lexer = Lexer::new(&content);
        let mut tokens = lexer.into_iter();

        // Iterate over expected tokens and actual tokens in a single pass.
        for expected_str in &test.expected {
            match tokens.next() {
                Some(actual_token) => {
                    // Assumes `into_str()` can be called on a reference, avoiding a clone.
                    let actual_str = actual_token.kind.into_str();
                    if actual_str != expected_str.as_str() {
                        error!(
                            "{}:{}:{}: Token mismatch\n\tExpected: {}\n\tGot:      {}",
                            test.file,
                            actual_token.loc.line,
                            actual_token.loc.col,
                            expected_str,
                            actual_str
                        );
                        continue 'tests; // Move to the next test file.
                    }
                }
                None => {
                    error!(
                        "{}: Unexpected end of input. Expected more tokens.",
                        test.file
                    );
                    continue 'tests;
                }
            }
        }

        // Check for any extra, unexpected tokens from the lexer.
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
        // Use format specifiers for efficient, clean padding.
        let width = longest_name;
        info!("{:>width$} passed", test.file);
    }

    println!("----------------------------------");
    info!("{}/{} tests passed", passed, total);

    Ok(())
}

impl serde::Serialize for Tests {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Tests", 1)?;
        state.serialize_field("tests", &self.tests)?;
        state.end()
    }
}

impl serde::Serialize for Test {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Test", 1)?;
        state.serialize_field("file", &self.file)?;
        state.serialize_field("expected", &self.expected)?;
        state.end()
    }
}

pub fn build_test_compiler() -> anyhow::Result<()> {
    let test_path = Path::new("./tests/");
    let expected_json_path = test_path.join("expected.json");

    let paths_to_process: Vec<PathBuf> = fs::read_dir(test_path)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file() && *path != expected_json_path)
        .collect();

    println!("---------------------------------------");
    let collected_tests: Vec<Test> = paths_to_process
        .into_par_iter()
        .map(|path| {
            let content = fs::read_to_string(&path)?;
            let mut test = Test::empty();
            test.file = path.to_str().unwrap_or("").to_string();

            let lexer = Lexer::new(&content);
            test.expected = lexer
                .into_iter()
                .map(|token| token.kind.into_str().to_string())
                .collect();

            info!("Test built: {}", path.display());
            Ok(test)
        })
        .collect::<Result<Vec<Test>>>()?;
    println!("---------------------------------------");

    let tests = Tests {
        tests: collected_tests,
    };
    let expected_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&expected_json_path)?;
    let mut writer = BufWriter::new(expected_file);

    serde_json::to_writer_pretty(&mut writer, &tests)?;

    Ok(())
}
