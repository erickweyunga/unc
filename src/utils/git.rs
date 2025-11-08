use anyhow::Result;
use colored::*;
use std::path::Path;
use std::process::Command;

/// Initializes a git repository in the specified directory
///
/// # Arguments
///
/// * `project_path` - Path to the project directory
///
/// # Returns
///
/// Returns `Ok(())` even if git initialization fails (with a warning),
/// as this is not a critical operation
pub fn init_git_repo(project_path: &Path) -> Result<()> {
    // Check if git is available first
    if !is_git_available() {
        eprintln!("{}", "Warning: git not found, skipping git init".yellow());
        return Ok(());
    }

    let output = Command::new("git")
        .args(["init"])
        .current_dir(project_path)
        .output();

    match output {
        Ok(output) if output.status.success() => {
            // Optionally create initial commit
            let _ = create_initial_commit(project_path);
            Ok(())
        }
        Ok(_) => {
            eprintln!(
                "{}",
                "Warning: Failed to initialize git repository".yellow()
            );
            Ok(())
        }
        Err(_) => {
            eprintln!("{}", "Warning: git not found, skipping git init".yellow());
            Ok(())
        }
    }
}

/// Creates an initial commit in the git repository
///
/// # Arguments
///
/// * `project_path` - Path to the project directory
///
/// # Returns
///
/// Returns `Ok(())` if successful, or an error if the commit fails
fn create_initial_commit(project_path: &Path) -> Result<()> {
    // Add all files
    Command::new("git")
        .args(["add", "."])
        .current_dir(project_path)
        .output()?;

    // Create initial commit
    Command::new("git")
        .args(["commit", "-m", "Initial commit from unc"])
        .current_dir(project_path)
        .output()?;

    Ok(())
}

/// Checks if git is available on the system
///
/// # Returns
///
/// `true` if git is available, `false` otherwise
pub fn is_git_available() -> bool {
    Command::new("git")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_git_available() {
        // This test will pass or fail depending on whether git is installed
        // We just ensure the function doesn't panic
        let _ = is_git_available();
    }
}
