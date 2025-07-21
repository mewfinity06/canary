use std::path::{self, PathBuf};

use anyhow;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    file: String,
}

impl Cli {
    pub fn get_abs_path(&self) -> anyhow::Result<PathBuf> {
        let abs = path::absolute(self.file.as_str())?;
        return Ok(abs);
    }
}
