pub mod create_app;
pub mod dev;

use anyhow::Result;

use crate::cli::Commands;
pub use create_app::create_app;
pub use dev::dev;

/// Dispatches commands to their respective handlers
///
/// # Arguments
///
/// * `command` - The parsed command from CLI
///
/// # Returns
///
/// Returns `Ok(())` if the command executes successfully, or an error otherwise
pub fn dispatch(command: Commands) -> Result<()> {
    match command {
        Commands::CreateApp {
            name,
            template,
            repo,
            branch,
        } => create_app(&name, &template, repo.as_deref(), &branch),
        Commands::Dev => dev(),
    }
}
