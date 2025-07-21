#![allow(unused_imports)]

// Rust imports
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

// Vendor imports
use clap::Parser as _;

// Canary mods
mod canary;
mod cli;
mod compiler_test;
mod lexer;
mod parser;

// Canary imports
use crate::lexer::Lexer;
use crate::lexer::token::Token;
use crate::parser::Parser;
use crate::parser::node::Node;
use compiler_test as ct;

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();

    let res = match cli.command {
        cli::Command::Run { .. } => run_file(cli.get_abs_path()?),
        cli::Command::TestCompiler => ct::test_compiler(),
        cli::Command::BuildTests => ct::build_tests(),
    };

    match res {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("{}", e);
            Ok(())
        }
    }
}

fn run_file(file: PathBuf) -> anyhow::Result<()> {
    let mut reader = BufReader::new(File::open(&file)?);
    let mut content = String::with_capacity(100);
    reader.read_to_string(&mut content)?;

    let lexer = Lexer::new(&content);
    let parser = Parser::new(lexer);

    for node in parser {
        info!("Found node: {:#?}", node);
    }

    Ok(())
}
