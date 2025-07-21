// Rust imports
use std::fs::File;
use std::io::{BufReader, Read};

// Vendor imports
use anyhow;
use clap::Parser;

// Canary mods
mod canary;
mod cli;
mod lexer;

// Canary imports
use crate::lexer::Lexer;
use crate::lexer::token::Token;

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();

    let abs_path = cli.get_abs_path()?;

    let mut reader = BufReader::new(File::open(&abs_path)?);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    // Lex tokens
    let mut lexer = Lexer::new(
        abs_path
            .to_str()
            .expect("this 'to_str' should always pass")
            .to_string(),
        &content,
    );

    loop {
        let token = match lexer.next() {
            Ok(t) => t,
            Err(e) => {
                error!("{}", e);
                break;
            }
        };

        if let Token::Invalid(c) = token {
            error!("Invalid character: {c}");
            break;
        } else if Token::EOF == token {
            break;
        } else {
            info!("Found: {:?}", token);
        }
    }

    Ok(())
}
