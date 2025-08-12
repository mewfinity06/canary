#![allow(dead_code)]

use std::{
    fs::{self, File, OpenOptions},
    io::{BufReader, BufWriter, Read},
    path::{Path, PathBuf},
};

use rayon::prelude::*;
use serde::{Serialize, Deserialize};

use utils::{error, info};
use lexer::Lexer;

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
    tests: Vec<Test>
}

impl Test {
    pub fn empty() -> Self {
        Self {
            tests: Vec::new(),
        }
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

    if USE_README_TESTS {
        build_readme_test()?;
    }

    let collected_tests: Vec<Test> = paths_to_process
        .into_par_iter()
        .map(|path| {
            info!("");
            let content = fs::read_to_string(&path)?;
            let mut test = Test::empty();
            test.file = path.to_str().unwrap_or("").to_string();

            // token.kind.into_str().to_string()

            let lexer = Lexer::new(&content);
            test.expected = lexer
                .into_iter()
                .map(|token| {
                    match token.kind {
                        TokenType::Error(e) => {
                            error!("{e}");
                            
                        }
                        _ => token.kind.into_str().to_string()
                    }
                })
                .collect();

            Ok(test)
        })
        .collect::<Result<Vec<Test>>>()?;
}

pub fn run_tests() -> anyhow::Result<()> {
    todo!();
}

pub fn build_and_run_tests() -> anyhow::Result<()> {
    todo!();
}

fn build_readme_test() -> anyhow::Result<()> {
    todo!();
}

fn run_readme_test() -> anyhow::Result<()> {
    todo!()
}