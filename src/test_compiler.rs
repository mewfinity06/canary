use anyhow::bail;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File, OpenOptions},
    io::{BufReader, BufWriter, Read},
};

use crate::{
    info,
    lexer::{Lexer, token::Token},
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
            expected: Vec::new()
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

        let mut lexer = Lexer::new(test.file.clone(), &content).into_iter();

        for e in test.expected {
            let t = lexer
                .next()
                .expect("There should always be a token present");
            if e != t.into_str() {
                bail!("");
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

        info!("Entry: {}", path.display());

        if path.to_str().expect("this to_str should always pass") == "./tests/expected.json" {
            continue;
        }
        
        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;

        let mut test = Test::empty();

        let mut lexer = Lexer::new(
            path.to_str()
                .expect("this to_str should always pass")
                .to_string(),
            &content,
        );

        loop {
            let t = lexer
                .next()
                .expect("There should always be a token present");
            if t == Token::EOF {
                break;
            }
            test.expected.push(t.into_str().to_string());
        }

        tests.tests.push(test);
    }

    serde_json::to_writer_pretty(&mut writer, &tests)?;

    Ok(())
}
