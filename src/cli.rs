#![allow(dead_code)]

use std::path::{self, PathBuf};

use anyhow::bail;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

impl Cli {
    pub fn get_abs_path(&self) -> anyhow::Result<PathBuf> {
        match &self.command {
            Command::Run { file } => {
                let abs = path::absolute(file)?;
                Ok(PathBuf::from(abs))
            }
            _ => bail!("Can only get absolute path from Run command"),
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Run { file: String },
    TestCompiler,
    BuildTests,
}
