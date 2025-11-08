use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::utils::copy_dir_recursively;

/// Downloads a template from a GitHub repository
///
/// # Arguments
///
/// * `repo_url` - Full GitHub repository URL or shorthand (username/repo)
/// * `branch` - Branch name to download from
/// * `template` - Template name (directory name in the repo)
/// * `dest` - Destination path where the template should be extracted
///
/// # Returns
///
/// Returns `Ok(())` if successful, or an error if download fails
pub fn download_template(repo_url: &str, branch: &str, template: &str, dest: &Path) -> Result<()> {
    // Parse repository information
    let (owner, repo) = parse_repo_url(repo_url)?;

    // GitHub API URL to get the tarball
    let tarball_url = format!(
        "https://api.github.com/repos/{}/{}/tarball/{}",
        owner, repo, branch
    );

    // Download tarball
    let bytes = download_tarball(&tarball_url)?;

    // Extract to temporary directory
    let temp_dir = tempfile::tempdir()?;
    extract_tarball(&bytes, temp_dir.path())?;

    // Find and copy the template
    copy_template_to_dest(temp_dir.path(), template, dest)?;

    Ok(())
}

/// Parses a GitHub repository URL or shorthand into owner and repo name
///
/// # Arguments
///
/// * `repo_url` - GitHub repository URL or shorthand
///
/// # Returns
///
/// Returns a tuple of (owner, repo) or an error if parsing fails
///
/// # Examples
///
/// ```
/// let (owner, repo) = parse_repo_url("https://github.com/user/repo").unwrap();
/// assert_eq!(owner, "user");
/// assert_eq!(repo, "repo");
///
/// let (owner, repo) = parse_repo_url("user/repo").unwrap();
/// assert_eq!(owner, "user");
/// assert_eq!(repo, "repo");
/// ```
fn parse_repo_url(repo_url: &str) -> Result<(&str, &str)> {
    let parts: Vec<&str> = repo_url.trim_end_matches('/').split('/').collect();

    if parts.len() >= 2 {
        let owner = parts[parts.len() - 2];
        let repo = parts[parts.len() - 1];
        Ok((owner, repo))
    } else {
        anyhow::bail!("Invalid repository URL: {}", repo_url);
    }
}

/// Downloads a tarball from GitHub API
///
/// # Arguments
///
/// * `tarball_url` - GitHub API tarball URL
///
/// # Returns
///
/// Returns the downloaded bytes or an error
fn download_tarball(tarball_url: &str) -> Result<Vec<u8>> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("unc-cli")
        .build()?;

    let response = client
        .get(tarball_url)
        .send()
        .context("Failed to download template")?;

    if !response.status().is_success() {
        anyhow::bail!(
            "Failed to download template: HTTP {}. Make sure the repository and branch exist.",
            response.status()
        );
    }

    let bytes = response.bytes()?.to_vec();
    Ok(bytes)
}

/// Extracts a gzipped tarball to a destination directory
///
/// # Arguments
///
/// * `bytes` - Tarball bytes
/// * `dest` - Destination directory path
fn extract_tarball(bytes: &[u8], dest: &Path) -> Result<()> {
    let tar = flate2::read::GzDecoder::new(bytes);
    let mut archive = tar::Archive::new(tar);
    archive.unpack(dest).context("Failed to extract tarball")?;
    Ok(())
}

/// Copies a template from the extracted directory to the destination
///
/// # Arguments
///
/// * `extracted_dir` - Directory where tarball was extracted
/// * `template` - Template name to find
/// * `dest` - Final destination directory
fn copy_template_to_dest(extracted_dir: &Path, template: &str, dest: &Path) -> Result<()> {
    // Find the template directory
    let extracted_dirs: Vec<_> = fs::read_dir(extracted_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .collect();

    if extracted_dirs.is_empty() {
        anyhow::bail!("No directories found in downloaded template");
    }

    let template_base = &extracted_dirs[0].path();
    let template_path = template_base.join(template);

    if !template_path.exists() {
        anyhow::bail!(
            "Template '{}' not found in repository. Available templates should be in the root directory.",
            template
        );
    }

    // Copy template files to destination
    copy_dir_recursively(&template_path, dest)?;

    Ok(())
}

/// Converts a repository shorthand to a full GitHub URL
///
/// # Arguments
///
/// * `repo` - Repository shorthand or full URL
///
/// # Returns
///
/// Full GitHub repository URL
///
/// # Examples
///
/// ```
/// let url = normalize_repo_url("user/repo");
/// assert_eq!(url, "https://github.com/user/repo");
///
/// let url = normalize_repo_url("https://github.com/user/repo");
/// assert_eq!(url, "https://github.com/user/repo");
/// ```
pub fn normalize_repo_url(repo: &str) -> String {
    if repo.starts_with("http://") || repo.starts_with("https://") {
        repo.to_string()
    } else {
        format!("https://github.com/{}", repo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_repo_url() {
        let (owner, repo) = parse_repo_url("https://github.com/user/repo").unwrap();
        assert_eq!(owner, "user");
        assert_eq!(repo, "repo");

        let (owner, repo) = parse_repo_url("user/repo").unwrap();
        assert_eq!(owner, "user");
        assert_eq!(repo, "repo");

        let (owner, repo) = parse_repo_url("https://github.com/org/my-repo/").unwrap();
        assert_eq!(owner, "org");
        assert_eq!(repo, "my-repo");
    }

    #[test]
    fn test_parse_repo_url_invalid() {
        assert!(parse_repo_url("invalid").is_err());
        assert!(parse_repo_url("").is_err());
    }

    #[test]
    fn test_normalize_repo_url() {
        assert_eq!(
            normalize_repo_url("user/repo"),
            "https://github.com/user/repo"
        );
        assert_eq!(
            normalize_repo_url("https://github.com/user/repo"),
            "https://github.com/user/repo"
        );
        assert_eq!(
            normalize_repo_url("http://github.com/user/repo"),
            "http://github.com/user/repo"
        );
    }
}
