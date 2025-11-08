use anyhow::Result;
use regex::Regex;
use std::fs;
use std::io::Write;
use std::path::Path;
use walkdir::WalkDir;

use crate::utils::should_skip_path;

/// Replaces placeholders in template files with actual values
///
/// # Arguments
///
/// * `project_path` - Path to the project directory
/// * `project_name` - Name of the project to replace placeholders with
///
/// # Returns
///
/// Returns `Ok(())` if successful, or an error if file operations fail
///
/// # Examples
///
/// This function will replace all occurrences of `{{project_name}}` in text files
/// with the actual project name.
pub fn replace_placeholders(project_path: &Path, project_name: &str) -> Result<()> {
    let placeholder_regex = Regex::new(r"\{\{project_name\}\}").unwrap();

    for entry in WalkDir::new(project_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();

        // Skip binary files and target directory
        if should_skip_path(path) {
            continue;
        }

        // Try to read file as text
        if let Ok(content) = fs::read_to_string(path) {
            // Replace placeholders
            let new_content = placeholder_regex.replace_all(&content, project_name);

            // Write back if changes were made
            if content != new_content.as_ref() {
                let mut file = fs::File::create(path)?;
                file.write_all(new_content.as_bytes())?;
            }
        }
    }

    Ok(())
}

/// Replaces multiple placeholders in template files
///
/// # Arguments
///
/// * `project_path` - Path to the project directory
/// * `replacements` - Map of placeholder names to their replacement values
///
/// # Returns
///
/// Returns `Ok(())` if successful, or an error if file operations fail
///
/// # Examples
///
/// ```no_run
/// use std::collections::HashMap;
/// use std::path::Path;
///
/// let mut replacements = HashMap::new();
/// replacements.insert("project_name".to_string(), "my-app".to_string());
/// replacements.insert("author".to_string(), "John Doe".to_string());
///
/// replace_multiple_placeholders(Path::new("./project"), &replacements).unwrap();
/// ```
#[allow(dead_code)]
pub fn replace_multiple_placeholders(
    project_path: &Path,
    replacements: &std::collections::HashMap<String, String>,
) -> Result<()> {
    for entry in WalkDir::new(project_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();

        // Skip binary files and target directory
        if should_skip_path(path) {
            continue;
        }

        // Try to read file as text
        if let Ok(mut content) = fs::read_to_string(path) {
            let original_content = content.clone();

            // Replace each placeholder
            for (key, value) in replacements {
                let placeholder = format!("{{{{{}}}}}", key);
                content = content.replace(&placeholder, value);
            }

            // Write back if changes were made
            if content != original_content {
                let mut file = fs::File::create(path)?;
                file.write_all(content.as_bytes())?;
            }
        }
    }

    Ok(())
}

/// Processes template files by applying transformations
///
/// # Arguments
///
/// * `project_path` - Path to the project directory
/// * `processor` - Function that processes each file's content
///
/// # Returns
///
/// Returns `Ok(())` if successful, or an error if file operations fail
#[allow(dead_code)]
pub fn process_template_files<F>(project_path: &Path, mut processor: F) -> Result<()>
where
    F: FnMut(&str) -> String,
{
    for entry in WalkDir::new(project_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();

        // Skip binary files and target directory
        if should_skip_path(path) {
            continue;
        }

        // Try to read file as text
        if let Ok(content) = fs::read_to_string(path) {
            let new_content = processor(&content);

            // Write back if changes were made
            if content != new_content {
                let mut file = fs::File::create(path)?;
                file.write_all(new_content.as_bytes())?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_replace_placeholders() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        // Create a test file with placeholder
        fs::write(&file_path, "Project: {{project_name}}").unwrap();

        // Replace placeholders
        replace_placeholders(temp_dir.path(), "my-app").unwrap();

        // Verify replacement
        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "Project: my-app");
    }

    #[test]
    fn test_replace_multiple_placeholders() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        // Create a test file with multiple placeholders
        fs::write(&file_path, "Project: {{project_name}}\nAuthor: {{author}}").unwrap();

        // Create replacements map
        let mut replacements = std::collections::HashMap::new();
        replacements.insert("project_name".to_string(), "my-app".to_string());
        replacements.insert("author".to_string(), "John Doe".to_string());

        // Replace placeholders
        replace_multiple_placeholders(temp_dir.path(), &replacements).unwrap();

        // Verify replacement
        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "Project: my-app\nAuthor: John Doe");
    }

    #[test]
    fn test_no_replacement_needed() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        // Create a test file without placeholders
        let original_content = "No placeholders here";
        fs::write(&file_path, original_content).unwrap();

        // Try to replace placeholders
        replace_placeholders(temp_dir.path(), "my-app").unwrap();

        // Verify content unchanged
        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, original_content);
    }

    #[test]
    fn test_process_template_files() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        // Create a test file
        fs::write(&file_path, "hello world").unwrap();

        // Process with uppercase transformation
        process_template_files(temp_dir.path(), |content| content.to_uppercase()).unwrap();

        // Verify transformation
        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "HELLO WORLD");
    }
}
