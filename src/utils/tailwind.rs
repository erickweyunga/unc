use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct TailwindConfig {
    #[serde(rename = "tw-input")]
    pub input: Vec<String>,
    #[serde(rename = "tw-output")]
    pub output: String,
    #[serde(rename = "tw-watch-enabled", default)]
    pub watch_enabled: bool,
    #[serde(rename = "tw-watch-always", default)]
    pub watch_always: bool,
    #[serde(rename = "tw-optimize-minify", default)]
    pub optimize_minify: bool,
    #[serde(rename = "tw-optimize-map", default)]
    pub optimize_map: bool,
}

#[derive(Debug, Deserialize)]
struct CargoToml {
    package: Option<Package>,
}

#[derive(Debug, Deserialize)]
struct Package {
    metadata: Option<Metadata>,
}

#[derive(Debug, Deserialize)]
struct Metadata {
    tailwind: Option<TailwindConfig>,
}

/// Reads and parses Tailwind CSS configuration from Cargo.toml
///
/// # Arguments
///
/// * `cargo_toml_path` - Path to the Cargo.toml file (defaults to "./Cargo.toml")
///
/// # Returns
///
/// Returns `Some(TailwindConfig)` if configuration exists, `None` otherwise
pub fn read_tailwind_config<P: AsRef<Path>>(cargo_toml_path: P) -> Result<Option<TailwindConfig>> {
    let path = cargo_toml_path.as_ref();

    if !path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(path).context("Failed to read Cargo.toml")?;

    let cargo_toml: CargoToml = toml::from_str(&content).context("Failed to parse Cargo.toml")?;

    Ok(cargo_toml
        .package
        .and_then(|p| p.metadata)
        .and_then(|m| m.tailwind))
}

/// Checks if Tailwind CSS is configured and enabled in the current project
///
/// # Returns
///
/// Returns `true` if Tailwind is configured and watch is enabled
pub fn is_tailwind_enabled() -> bool {
    read_tailwind_config("Cargo.toml")
        .ok()
        .flatten()
        .map(|config| config.watch_enabled)
        .unwrap_or(false)
}

/// Builds the Tailwind CSS command arguments based on configuration
///
/// # Arguments
///
/// * `config` - The Tailwind configuration
///
/// # Returns
///
/// Returns a vector of command arguments for the tailwindcss CLI
pub fn build_tailwind_args(config: &TailwindConfig) -> Vec<String> {
    let mut args = Vec::new();

    // Add input file (use first one if multiple)
    if let Some(input) = config.input.first() {
        args.push("-i".to_string());
        args.push(input.clone());
    }

    // Add output file
    args.push("-o".to_string());
    args.push(config.output.clone());

    // Add watch flag if enabled
    if config.watch_enabled {
        if config.watch_always {
            args.push("-w=always".to_string());
        } else {
            args.push("-w".to_string());
        }
    }

    // Add optimization flags
    if config.optimize_minify {
        args.push("-m".to_string());
    }

    // Add source map flag
    if config.optimize_map {
        args.push("--map".to_string());
    }

    args
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_tailwind_args() {
        let config = TailwindConfig {
            input: vec!["src/styles/tailwind.css".to_string()],
            output: "public/output.css".to_string(),
            watch_enabled: true,
            watch_always: true,
            optimize_minify: true,
            optimize_map: false,
        };

        let args = build_tailwind_args(&config);

        assert!(args.contains(&"-i".to_string()));
        assert!(args.contains(&"src/styles/tailwind.css".to_string()));
        assert!(args.contains(&"-o".to_string()));
        assert!(args.contains(&"public/output.css".to_string()));
        assert!(args.contains(&"-w=always".to_string()));
        assert!(args.contains(&"-m".to_string()));
        assert!(!args.contains(&"--map".to_string()));
    }

    #[test]
    fn test_build_tailwind_args_minimal() {
        let config = TailwindConfig {
            input: vec!["input.css".to_string()],
            output: "output.css".to_string(),
            watch_enabled: false,
            watch_always: false,
            optimize_minify: false,
            optimize_map: false,
        };

        let args = build_tailwind_args(&config);

        assert!(args.contains(&"-i".to_string()));
        assert!(args.contains(&"input.css".to_string()));
        assert!(args.contains(&"-o".to_string()));
        assert!(args.contains(&"output.css".to_string()));
        assert!(!args.contains(&"-w".to_string()));
        assert!(!args.contains(&"-w=always".to_string()));
        assert!(!args.contains(&"-m".to_string()));
    }
}
