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
        cli::Command::TestCompiler => tester::test_compiler(),
        cli::Command::BuildTests => tester::build_test_compiler(),
        cli::Command::BuildAndTestCompiler => {
            tester::build_test_compiler()?;
            tester::test_compiler()
        }
    };

    match res {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("{e}");
            Ok(())
        }
    }
}
