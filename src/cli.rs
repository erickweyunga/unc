use clap::{Parser, Subcommand};

/// Uncovr CLI - Scaffold web applications with ease
#[derive(Parser)]
#[command(name = "unc")]
#[command(about = "A CLI tool for Uncovr", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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

    /// Run the project with cargo watch
    Dev,
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
