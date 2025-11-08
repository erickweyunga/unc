use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Uncovr CLI - Scaffold web applications with ease
#[derive(Parser)]
#[command(name = "unc")]
#[command(about = "A CLI tool for scaffolding Uncovr applications", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new application from a template
    #[command(name = "create-app")]
    CreateApp {
        /// Name of the application
        name: String,

        /// Template to use (default: default)
        #[arg(short, long, default_value = "default")]
        template: String,

        /// GitHub repository URL or shorthand (e.g., username/repo)
        #[arg(short, long)]
        repo: Option<String>,

        /// Branch to use (default: main)
        #[arg(short, long, default_value = "main")]
        branch: String,
    },
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::CreateApp {
            name,
            template,
            repo,
            branch,
        } => create_app(&name, &template, repo.as_deref(), &branch),
    }
}

fn create_app(name: &str, template: &str, repo: Option<&str>, branch: &str) -> Result<()> {
    // Validate project name
    validate_project_name(name)?;

    // Determine repository URL
    let repo_url = repo.unwrap_or("erickweyunga/uncovr-templates");
    let full_repo_url = if repo_url.starts_with("http") {
        repo_url.to_string()
    } else {
        format!("https://github.com/{}", repo_url)
    };

    println!(
        "{} {}",
        "Creating project:".green().bold(),
        name.cyan().bold()
    );
    println!("{} {}", "Using template:".green(), template.cyan());
    println!("{} {}", "From repository:".green(), full_repo_url.cyan());
    println!();

    // Create project directory
    let project_path = PathBuf::from(name);
    if project_path.exists() {
        anyhow::bail!("Directory '{}' already exists", name);
    }

    fs::create_dir_all(&project_path)
        .context(format!("Failed to create directory '{}'", name))?;

    // Download template
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.set_message("Downloading template...");
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    download_template(&full_repo_url, branch, template, &project_path)?;

    pb.finish_with_message(format!("{}", "Template downloaded!".green()));

    // Replace placeholders
    println!("{}", "Replacing placeholders...".green());
    replace_placeholders(&project_path, name)?;

    // Initialize git repository
    println!("{}", "Initializing git repository...".green());
    init_git_repo(&project_path)?;

    println!();
    println!("{}", "✨ Project created successfully!".green().bold());
    println!();
    println!("Next steps:");
    println!("  {} {}", "cd".cyan(), name.cyan());
    println!("  {} {}", "cargo".cyan(), "run".cyan());
    println!();

    Ok(())
}

fn validate_project_name(name: &str) -> Result<()> {
    let re = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_-]*$").unwrap();
    if !re.is_match(name) {
        anyhow::bail!(
            "Invalid project name '{}'. Must start with a letter and contain only letters, numbers, hyphens, and underscores.",
            name
        );
    }
    Ok(())
}

fn download_template(repo_url: &str, branch: &str, template: &str, dest: &Path) -> Result<()> {
    // Extract owner and repo from URL
    let parts: Vec<&str> = repo_url
        .trim_end_matches('/')
        .split('/')
        .collect();
    
    let (owner, repo) = if parts.len() >= 2 {
        (parts[parts.len() - 2], parts[parts.len() - 1])
    } else {
        anyhow::bail!("Invalid repository URL: {}", repo_url);
    };

    // GitHub API URL to get the tarball
    let tarball_url = format!(
        "https://api.github.com/repos/{}/{}/tarball/{}",
        owner, repo, branch
    );

    // Download tarball
    let client = reqwest::blocking::Client::builder()
        .user_agent("unc-cli")
        .build()?;

    let response = client
        .get(&tarball_url)
        .send()
        .context("Failed to download template")?;

    if !response.status().is_success() {
        anyhow::bail!(
            "Failed to download template: HTTP {}. Make sure the repository and branch exist.",
            response.status()
        );
    }

    let bytes = response.bytes()?;

    // Create temporary directory
    let temp_dir = tempfile::tempdir()?;

    // Extract tarball
    let tar = flate2::read::GzDecoder::new(&bytes[..]);
    let mut archive = tar::Archive::new(tar);
    archive.unpack(temp_dir.path())?;

    // Find the template directory
    let extracted_dirs: Vec<_> = fs::read_dir(temp_dir.path())?
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

fn copy_dir_recursively(src: &Path, dst: &Path) -> Result<()> {
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

fn replace_placeholders(project_path: &Path, project_name: &str) -> Result<()> {
    let placeholder_regex = Regex::new(r"\{\{project_name\}\}").unwrap();

    for entry in WalkDir::new(project_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();

        // Skip binary files and target directory
        if path.to_string_lossy().contains("/target/")
            || path
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| matches!(s, "exe" | "dll" | "so" | "dylib"))
                .unwrap_or(false)
        {
            continue;
        }

        // Read file content
        if let Ok(content) = fs::read_to_string(path) {
            // Replace placeholders
            let new_content = placeholder_regex.replace_all(&content, project_name);

            // Write back if changes were made
            if content != new_content {
                let mut file = fs::File::create(path)?;
                file.write_all(new_content.as_bytes())?;
            }
        }
    }

    Ok(())
}

fn init_git_repo(project_path: &Path) -> Result<()> {
    use std::process::Command;

    let output = Command::new("git")
        .args(["init"])
        .current_dir(project_path)
        .output();

    match output {
        Ok(output) if output.status.success() => Ok(()),
        Ok(_) => {
            eprintln!("{}", "Warning: Failed to initialize git repository".yellow());
            Ok(())
        }
        Err(_) => {
            eprintln!("{}", "Warning: git not found, skipping git init".yellow());
            Ok(())
        }
    }
}
