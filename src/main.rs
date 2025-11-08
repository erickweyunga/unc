mod cli;
mod commands;
mod template;
mod utils;

use anyhow::Result;
use colored::*;

use cli::Cli;
use commands::dispatch;

fn main() {
    if let Err(e) = run() {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse_args();
    dispatch(cli.command)
}
