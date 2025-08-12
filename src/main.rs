mod cli;
mod runner;
mod tester2;

use utils::error;

use anyhow::Result;
use clap::Parser as ClapParser;
use cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let res = match cli.command {
        cli::Command::Run { .. } => runner::run_file(&cli),
        cli::Command::BuildTests => tester2::build_tests(),
        cli::Command::RunTests => tester2::run_tests(),
        cli::Command::BuildAndRunTests => tester2::build_and_run_tests(),
        cli::Command::BuildReadmeTests => tester2::build_readme_test(),
        cli::Command::RunReadmeTests => tester2::run_readme_test(),
    };

    match res {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("{e}");
            Ok(())
        }
    }
}
