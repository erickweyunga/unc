use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;

use crate::template::{download_template, normalize_repo_url, replace_placeholders};
use crate::utils::{
    ensure_cargo_watch, ensure_directory, get_run_command, init_git_repo, validate_project_name,
};

/// Creates a new application from a template
///
/// # Arguments
///
/// * `name` - Name of the application
/// * `template` - Template to use
/// * `repo` - Optional GitHub repository URL or shorthand
/// * `branch` - Branch to use from the repository
///
/// # Returns
///
/// Returns `Ok(())` if successful, or an error if creation fails
pub fn create_app(name: &str, template: &str, repo: Option<&str>, branch: &str) -> Result<()> {
    // Validate project name
    validate_project_name(name)?;

    // Determine repository URL
    let repo_url = repo.unwrap_or("erickweyunga/uncovr-templates");
    let full_repo_url = normalize_repo_url(repo_url);

    // Print creation info
    print_creation_info();

    // Create project directory
    let project_path = PathBuf::from(name);
    check_directory_exists(&project_path, name)?;
    ensure_directory(&project_path, name)?;

    // Download template with progress indicator
    let pb = create_progress_bar();
    pb.set_message("Loading...");

    // Execute operations and cleanup on error
    let result = (|| -> Result<()> {
        download_template(&full_repo_url, branch, template, &project_path)?;
        replace_placeholders(&project_path, name)?;
        init_git_repo(&project_path)?;
        Ok(())
    })();

    // Cleanup on error
    if let Err(e) = result {
        pb.finish_and_clear();
        eprintln!("{} {}", "Error:".red().bold(), e);
        eprintln!("{}", "Cleaning up...".yellow());
        cleanup_directory(&project_path);
        return Err(e);
    }

    pb.finish_and_clear();

    // Ensure cargo-watch is installed
    let _ = ensure_cargo_watch();

    // Print success message
    print_success_message(name);

    Ok(())
}

/// Prints information about the project being created
fn print_creation_info() {
    println!("{}", "Setting up your project...".green().bold());
    println!();
}

/// Checks if a directory already exists and returns an error if it does
fn check_directory_exists(path: &PathBuf, name: &str) -> Result<()> {
    if path.exists() {
        anyhow::bail!("Directory '{}' already exists", name);
    }
    Ok(())
}

/// Creates a styled progress bar for template download
fn create_progress_bar() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.enable_steady_tick(std::time::Duration::from_millis(80));
    pb
}

/// Prints the success message after project creation
fn print_success_message(name: &str) {
    let run_cmd = get_run_command();

    println!();
    println!("{}", "Project created successfully!".green().bold());
    println!();
    println!("  cd {}", name.cyan());
    println!("  {}", run_cmd);
    println!();
}

/// Cleans up a directory (removes it) when an error occurs
fn cleanup_directory(path: &PathBuf) {
    use std::fs;
    if path.exists() {
        let _ = fs::remove_dir_all(path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_check_directory_exists_with_nonexistent() {
        let path = PathBuf::from("nonexistent_dir_12345");
        assert!(check_directory_exists(&path, "nonexistent_dir_12345").is_ok());
    }

    #[test]
    fn test_normalize_repo_url_integration() {
        let url = normalize_repo_url("user/repo");
        assert_eq!(url, "https://github.com/user/repo");
    }

    #[test]
    fn test_cleanup_directory() {
        let temp_dir = TempDir::new().unwrap();
        let test_path = temp_dir.path().join("test_project");

        // Create a directory
        fs::create_dir_all(&test_path).unwrap();
        assert!(test_path.exists());

        // Cleanup
        cleanup_directory(&test_path);

        // Verify it's removed
        assert!(!test_path.exists());
    }

    #[test]
    fn test_cleanup_directory_nonexistent() {
        let path = PathBuf::from("nonexistent_cleanup_test_12345");

        // Should not panic when directory doesn't exist
        cleanup_directory(&path);
    }
}
