use anyhow::Result;
use regex::Regex;

/// Validates a project name to ensure it follows naming conventions
///
/// Rules:
/// - Must start with a letter
/// - Can only contain letters, numbers, hyphens, and underscores
pub fn validate_project_name(name: &str) -> Result<()> {
    let re = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_-]*$").unwrap();
    if !re.is_match(name) {
        anyhow::bail!(
            "Invalid project name '{}'. Must start with a letter and contain only letters, numbers, hyphens, and underscores.",
            name
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_project_names() {
        assert!(validate_project_name("my-project").is_ok());
        assert!(validate_project_name("my_project").is_ok());
        assert!(validate_project_name("myProject").is_ok());
        assert!(validate_project_name("project123").is_ok());
        assert!(validate_project_name("a").is_ok());
    }

    #[test]
    fn test_invalid_project_names() {
        assert!(validate_project_name("123project").is_err());
        assert!(validate_project_name("-project").is_err());
        assert!(validate_project_name("_project").is_err());
        assert!(validate_project_name("my project").is_err());
        assert!(validate_project_name("my@project").is_err());
        assert!(validate_project_name("").is_err());
    }
}
