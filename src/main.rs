mod cli;
mod runner;
mod tester;

use utils::error;

use anyhow::Result;
use clap::Parser as ClapParser;
use cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let res = match cli.command {
        cli::Command::Run { .. } => runner::run_file(&cli),
        cli::Command::BuildTests => tester::build_tests(cli.verbose),
        cli::Command::RunTests => tester::run_tests(cli.verbose),
        cli::Command::BuildAndRunTests => tester::build_and_run_tests(cli.verbose),
        
    };

    match res {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("{e}");
            Ok(())
        }
    }
}
