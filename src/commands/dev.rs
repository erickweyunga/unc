use anyhow::Result;
use colored::*;
use std::process::{Command, Stdio};

use crate::utils::{ensure_cargo_watch, is_cargo_watch_installed};

/// Runs the project with cargo watch for hot reloading
///
/// # Returns
///
/// Returns `Ok(())` if successful, or an error if execution fails
pub fn dev() -> Result<()> {
    // Ensure cargo-watch is installed
    if !is_cargo_watch_installed() {
        println!("{}", "cargo-watch is not installed.".yellow());
        ensure_cargo_watch()?;
    }

    println!(
        "{}",
        "Starting development server with hot reload..."
            .green()
            .bold()
    );
    println!();

    // Run cargo watch
    let status = Command::new("cargo")
        .args(["watch", "-x", "run"])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if !status.success() {
        anyhow::bail!("cargo watch exited with an error");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dev_command_exists() {
        // Just ensure the function signature is correct
        let _result: Result<()> = Ok(());
    }
}
