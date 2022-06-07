#![allow(unused)]

use std::fmt::format;

use anyhow::{Context, Ok, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,

    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let path = "test.txt";
    // let args = Cli::parse();
    let content =
        std::fs::read_to_string(path).with_context(|| format!("could not read file: {}", path))?;

    Ok(())
}
