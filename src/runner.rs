#![allow(dead_code, unused_imports)]

use std::fs::File;
use std::io::{BufReader, Read};

use anyhow::{Result, bail};

use lexer::Lexer;
use parser::Parser;
use parser::node::*;
use utils::*;

use crate::cli::Cli;

pub fn run_file(cli: &Cli) -> Result<()> {
    let file = File::open(cli.get_abs_path()?)?;
    let mut buffer = String::new();
    BufReader::new(file).read_to_string(&mut buffer)?;

    let lexer = Lexer::new(&buffer);
    let mut parser = Parser::new(lexer);
    let program = parser.program()?;

    for node in program.0 {
        info!("Node: {:?}", node);
    }

    Ok(())
}
