// Rust imports
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

// Vendor imports
use clap::Parser;

// Canary mods
mod canary;
mod cli;
mod lexer;
mod test_compiler;

// Canary imports
use crate::lexer::Lexer;
use crate::lexer::token::Token;
use test_compiler as tc;

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();

    // let abs_path = cli.get_abs_path()?;

    // let mut reader = BufReader::new(File::open(&abs_path)?);
    // let mut content = String::new();
    // reader.read_to_string(&mut content)?;

    let res = match cli.command {
        cli::Command::Run { .. } => run_file(cli.get_abs_path()?),
        cli::Command::TestCompiler => tc::test_compiler(),
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

    let mut lexer = Lexer::new(
        file.to_str()
            .expect("this to_str should always pass")
            .to_string(),
        &content,
    );

    loop {
        let token = lexer.next().unwrap();

        if let Token::Invalid(c) = token {
            error!("Unknown character found: {c}");
            break;
        } else if Token::EOF == token {
            break;
        } else {
            info!("Found token: {token:?}");
        }
    }

    Ok(())
}
