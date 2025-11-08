use anyhow::Result;
use colored::*;
use std::process::Command;

/// Checks if cargo-watch is installed
///
/// # Returns
///
/// `true` if cargo-watch is installed, `false` otherwise
pub fn is_cargo_watch_installed() -> bool {
    Command::new("cargo")
        .args(["watch", "--version"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Installs cargo-watch using cargo install
///
/// # Returns
///
/// Returns `Ok(())` if installation succeeds, or an error if it fails
pub fn install_cargo_watch() -> Result<()> {
    println!("{}", "Installing cargo-watch...".yellow());

    let output = Command::new("cargo")
        .args(["install", "cargo-watch"])
        .output()?;

    if output.status.success() {
        println!("{}", "cargo-watch installed successfully!".green());
        Ok(())
    } else {
        anyhow::bail!("Failed to install cargo-watch");
    }
}

/// Ensures cargo-watch is installed, installing it if necessary
///
/// # Returns
///
/// Returns `Ok(())` if cargo-watch is available (either already installed or just installed)
pub fn ensure_cargo_watch() -> Result<()> {
    if !is_cargo_watch_installed() {
        println!("{}", "cargo-watch is not installed.".yellow());
        install_cargo_watch()?;
    }
    Ok(())
}

/// Gets the recommended run command based on cargo-watch availability
///
/// # Returns
///
/// Returns "cargo watch -x run" if cargo-watch is installed, otherwise "cargo run"
pub fn get_run_command() -> String {
    if is_cargo_watch_installed() {
        "cargo watch -x run".to_string()
    } else {
        "cargo run".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_cargo_watch_installed() {
        // This will check the actual system
        let result = is_cargo_watch_installed();
        // We just ensure it doesn't panic
        let _ = result;
    }

    #[test]
    fn test_get_run_command() {
        let cmd = get_run_command();
        assert!(cmd.contains("cargo"));
        assert!(cmd.contains("run"));
    }
}
