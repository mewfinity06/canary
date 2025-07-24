mod canary;
mod cli;
mod lexer;
mod parser;
mod runner;
mod tester;

use anyhow::Result;
use clap::Parser as ClapParser;
use cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let res = match cli.command {
        cli::Command::Run { .. } => runner::run_file(&cli),
        cli::Command::TestCompiler => tester::test_compiler(),
        cli::Command::BuildTests => tester::build_test_compiler(),
    };

    match res {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("{e}");
            Ok(())
        }
    }
}
