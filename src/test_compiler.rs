use anyhow::bail;
use serde::Deserialize;
use std::{
    fs::File,
    io::{BufReader, Read},
};

use crate::{
    info,
    lexer::{Lexer, token::Token},
};

#[derive(Deserialize, Debug)]
struct Tests {
    tests: Vec<Test>,
}

#[derive(Deserialize, Debug)]
struct Test {
    file: String,
    expected: Vec<String>,
}

pub fn test_compiler() -> anyhow::Result<()> {
    use Token::*;

    let expected = File::open("./tests/expected.json")?;
    let tests: Tests = serde_json::from_reader(expected)?;

    for test in tests.tests {
        let test_file = File::open(&test.file)?;
        let mut reader = BufReader::new(test_file);
        let mut content = std::string::String::with_capacity(256);
        reader.read_to_string(&mut content)?;

        let mut lexer = Lexer::new(test.file.clone(), &content).into_iter();

        for e in test.expected {
            let t = lexer.next().expect("There should always be a token present");
            match (e.as_str(), &t) {
                ("DotDotDot", DotDotDot) => {}
                ("Assign", Assign) => {}
                ("PlusEql", PlusEql) => {}
                ("MinusEql", MinusEql) => {}
                ("StarEql", StarEql) => {}
                ("DivEql", DivEql) => {}
                ("LessEql", LessEql) => {}
                ("GreaterEql", GreaterEql) => {}
                ("DoubleEql", DoubleEql) => {}
                ("RightArrow", RightArrow) => {}
                ("FatRightArrow", FatRightArrow) => {}
                ("Pipe", Pipe) => {}
                ("Colon", Colon) => {}
                ("SemiColon", SemiColon) => {}
                ("Eql", Eql) => {}
                ("Plus", Plus) => {}
                ("Minus", Minus) => {}
                ("Star", Star) => {}
                ("Div", Div) => {}
                ("Less", Less) => {}
                ("Greater", Greater) => {}
                ("VertBar", VertBar) => {}
                ("Dot", Dot) => {}
                ("Comma", Comma) => {}
                ("Question", Question) => {}
                ("Bang", Bang) => {}
                ("Pound", Pound) => {}
                ("OParen", OParen) => {}
                ("CParen", CParen) => {}
                ("OBrack", OBrack) => {}
                ("CBrack", CBrack) => {}
                ("OSquare", OSquare) => {}
                ("CSquare", CSquare) => {}
                ("Const", Const) => {}
                ("Val", Val) => {}
                ("Mut", Mut) => {}
                ("Struct", Struct) => {}
                ("Enum", Enum) => {}
                ("Macro", Macro) => {}
                ("Impl", Impl) => {}
                ("Interface", Interface) => {}
                ("Priv", Priv) => {}
                ("Pub", Pub) => {}
                ("Override", Override) => {}
                ("Fn", Fn) => {}
                ("Defer", Defer) => {}
                ("If", If) => {}
                ("Else", Else) => {}
                ("Switch", Switch) => {}
                ("For", For) => {}
                ("Break", Break) => {}
                ("Continue", Continue) => {}
                ("Unreachable", Unreachable) => {}
                ("Ident", Ident(_)) => {}
                ("Number", Number(_)) => {}
                ("String", String(_)) => {}
                ("Invalid", Invalid(_)) => {}
                ("EOF", EOF) => {}
                _ => bail!("{}: Expected {} but found {:?}", test.file, e, t),
            }
        }

        info!("{} passed!", test.file);
    }

    Ok(())
}
