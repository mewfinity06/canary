use anyhow::bail;
use serde::Deserialize;
use std::{
    fs::{self, File, OpenOptions},
    io::{BufReader, BufWriter, Read},
};

use crate::{
    info,
    lexer::{
        Lexer,
        token::{Token, TokenType},
    },
};

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
    let expected = File::open("./tests/expected.json")?;
    let tests: Tests = serde_json::from_reader(expected)?;

    for test in tests.tests {
        let test_file = File::open(&test.file)?;
        let mut reader = BufReader::new(test_file);
        let mut content = std::string::String::with_capacity(256);
        reader.read_to_string(&mut content)?;

        let lexer = Lexer::new(&content);

        for (i, e) in test.expected.iter().enumerate() {
            let t = lexer
                .clone()
                .into_iter()
                .nth(i)
                .expect("There should always be a token present");
            if e != t.kind.clone().into_str() {
                bail!("Token mismatch: expected {}, got {}", e, t.kind.into_str());
            }
        }

        info!("{} passed!", test.file);
    }

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

pub fn build_tests() -> anyhow::Result<()> {
    let test_path = "./tests/";
    let expected = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("./tests/expected.json")?;
    let mut writer = BufWriter::new(expected);

    let mut tests = Tests::empty();

    for entry in fs::read_dir(test_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        if path.to_str().expect("this to_str should always pass") == "./tests/expected.json" {
            continue;
        }

        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;

        let mut test = Test::empty();
        test.file = path
            .to_str()
            .expect("this to_str should always pass")
            .to_string();

        let mut lexer = Lexer::new(&content);

        while let Ok(t) = lexer.next_token() {
            if t.kind == TokenType::EOF {
                break;
            }
            test.expected.push(t.kind.into_str().to_string());
        }

        info!("Test {} built", path.display());
        tests.tests.push(test);
    }
    serde_json::to_writer_pretty(&mut writer, &tests)?;

    Ok(())
}
