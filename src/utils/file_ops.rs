use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// Copies a directory and all its contents recursively
///
/// # Arguments
///
/// * `src` - Source directory path
/// * `dst` - Destination directory path
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// copy_dir_recursively(Path::new("./source"), Path::new("./dest")).unwrap();
/// ```
pub fn copy_dir_recursively(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;

    for entry in WalkDir::new(src).min_depth(1) {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path.strip_prefix(src)?;
        let target_path = dst.join(relative_path);

        if entry.file_type().is_dir() {
            fs::create_dir_all(&target_path)?;
        } else {
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(path, &target_path)?;
        }
    }

    Ok(())
}

/// Checks if a path is a binary file based on its extension
///
/// # Arguments
///
/// * `path` - Path to check
///
/// # Returns
///
/// `true` if the file is likely binary, `false` otherwise
pub fn is_binary_file(path: &Path) -> bool {
    path.extension()
        .and_then(|s| s.to_str())
        .map(|s| matches!(s, "exe" | "dll" | "so" | "dylib" | "bin" | "o" | "a"))
        .unwrap_or(false)
}

/// Checks if a path should be skipped during processing
///
/// # Arguments
///
/// * `path` - Path to check
///
/// # Returns
///
/// `true` if the path should be skipped, `false` otherwise
pub fn should_skip_path(path: &Path) -> bool {
    let path_str = path.to_string_lossy();

    // Skip target directory
    if path_str.contains("/target/") || path_str.contains("\\target\\") {
        return true;
    }

    // Skip binary files
    if is_binary_file(path) {
        return true;
    }

    false
}

/// Creates a directory if it doesn't exist
///
/// # Arguments
///
/// * `path` - Directory path to create
/// * `name` - Name of the directory (for error messages)
pub fn ensure_directory(path: &Path, name: &str) -> Result<()> {
    fs::create_dir_all(path).context(format!("Failed to create directory '{}'", name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_binary_file() {
        assert!(is_binary_file(Path::new("file.exe")));
        assert!(is_binary_file(Path::new("lib.dll")));
        assert!(is_binary_file(Path::new("lib.so")));
        assert!(is_binary_file(Path::new("lib.dylib")));
        assert!(!is_binary_file(Path::new("file.txt")));
        assert!(!is_binary_file(Path::new("file.rs")));
        assert!(!is_binary_file(Path::new("Cargo.toml")));
    }

    #[test]
    fn test_should_skip_path() {
        assert!(should_skip_path(Path::new("src/target/debug/app")));
        assert!(should_skip_path(Path::new(
            "project\\target\\release\\app.exe"
        )));
        assert!(should_skip_path(Path::new("file.exe")));
        assert!(!should_skip_path(Path::new("src/main.rs")));
        assert!(!should_skip_path(Path::new("Cargo.toml")));
    }
}
