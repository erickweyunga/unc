pub mod download;
pub mod process;

// Re-export commonly used functions
pub use download::{download_template, normalize_repo_url};
pub use process::replace_placeholders;
