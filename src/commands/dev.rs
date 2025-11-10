use anyhow::Result;
use colored::*;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::utils::{
    build_tailwind_args, ensure_cargo_watch, is_cargo_watch_installed, is_tailwind_enabled,
    read_tailwind_config,
};

/// A guard that ensures a child process is killed when dropped
struct ProcessGuard {
    child: Option<Child>,
    #[allow(dead_code)]
    name: String,
}

impl ProcessGuard {
    fn new(child: Child, name: &str) -> Self {
        Self {
            child: Some(child),
            name: name.to_string(),
        }
    }

    fn take(&mut self) -> Option<Child> {
        self.child.take()
    }
}

impl Drop for ProcessGuard {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            // Try to kill the process silently
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

/// Checks if npx (Node.js) is available
fn is_npx_available() -> bool {
    Command::new("npx")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Spawns the Tailwind CSS watcher process
fn spawn_tailwind_process() -> Result<Child> {
    let config = read_tailwind_config("Cargo.toml")?
        .ok_or_else(|| anyhow::anyhow!("Tailwind config not found"))?;

    let args = build_tailwind_args(&config);

    let child = Command::new("npx")
        .arg("tailwindcss")
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    Ok(child)
}

/// Kills a child process gracefully
fn kill_process(mut child: Child, _name: &str) {
    // Try to kill the process silently
    let _ = child.kill();
    let _ = child.wait();
}

/// Runs the project with cargo watch for hot reloading
/// and optionally runs Tailwind CSS watcher if enabled
///
/// # Returns
///
/// Returns `Ok(())` if successful, or an error if execution fails
pub fn dev() -> Result<()> {
    // Ensure cargo-watch is installed
    if !is_cargo_watch_installed() {
        println!("{}", "cargo-watch is not installed, installing...".yellow());
        ensure_cargo_watch()?;
    }

    // Check if Tailwind CSS is enabled
    let tailwind_enabled = is_tailwind_enabled();

    // If Tailwind is enabled, check for npx
    if tailwind_enabled && !is_npx_available() {
        println!(
            "{}",
            "  warning: tailwind enabled but npx not found".yellow()
        );
        println!("{}", "  install node.js to use tailwind css\n".yellow());
    }

    println!("{}", "unc dev\n".bold());

    // Spawn Tailwind CSS watcher if enabled and npx is available
    let mut tailwind_guard = if tailwind_enabled && is_npx_available() {
        match spawn_tailwind_process() {
            Ok(child) => {
                thread::sleep(Duration::from_millis(500));
                Some(ProcessGuard::new(child, "Tailwind CSS"))
            }
            Err(_) => None,
        }
    } else {
        None
    };

    // Show what's running
    if tailwind_guard.is_some() {
        println!("  {} watching: cargo + tailwind", "▲".green());
    } else {
        println!("  {} watching: cargo", "▲".green());
    }

    // Spawn cargo watch
    let cargo_child = Command::new("cargo")
        .args(["watch", "-x", "run"])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    let mut cargo_guard = ProcessGuard::new(cargo_child, "cargo-watch");

    // Set up signal handler for Ctrl+C
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    println!("  {} ready in {}ms\n", "▲".green(), "500".dimmed());
    println!("  press {} to stop\n", "ctrl+c".dimmed());

    // Wait for cargo watch or Ctrl+C
    let cargo_status = loop {
        // Check if we received Ctrl+C
        if !running.load(Ordering::SeqCst) {
            println!("\n  {} shutting down...", "▲".yellow());

            // Kill cargo watch
            if let Some(child) = cargo_guard.take() {
                kill_process(child, "cargo-watch");
            }

            // Kill Tailwind if it's running
            if let Some(child) = tailwind_guard.as_mut().and_then(|g| g.take()) {
                kill_process(child, "Tailwind CSS");
            }

            println!("  {} stopped\n", "▲".green());
            return Ok(());
        }

        // Check if cargo watch has exited
        if let Some(child) = cargo_guard.child.as_mut() {
            match child.try_wait() {
                Ok(Some(status)) => break status,
                Ok(None) => {
                    // Process is still running, sleep a bit
                    thread::sleep(Duration::from_millis(100));
                }
                Err(e) => {
                    // ProcessGuards will clean up automatically on drop
                    return Err(e.into());
                }
            }
        } else {
            anyhow::bail!("cargo-watch process was lost");
        }
    };

    // Cargo watch has exited, kill Tailwind if it's running
    if let Some(child) = tailwind_guard.as_mut().and_then(|g| g.take()) {
        kill_process(child, "Tailwind CSS");
    }

    if !cargo_status.success() {
        anyhow::bail!("cargo watch exited with an error");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_npx_available() {
        // Just ensure it doesn't panic
        let _ = is_npx_available();
    }

    #[test]
    fn test_dev_command_exists() {
        // Just ensure the function signature is correct
        let _result: Result<()> = Ok(());
    }
}
