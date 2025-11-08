# CLI Documentation

A modular CLI tool for scaffolding Uncovr applications with hot reload support.

## Installation

```bash
cargo install --path .
```

## Commands

### create-app

Create a new application from a template.

**Usage:**
```bash
unc create-app <NAME> [OPTIONS]
```

**Arguments:**
- `<NAME>` - Name of the application

**Options:**
- `-t, --template <TEMPLATE>` - Template to use (default: "default")
- `-r, --repo <REPO>` - GitHub repository (default: "erickweyunga/uncovr-templates")
- `-b, --branch <BRANCH>` - Branch to use (default: "main")

**Examples:**
```bash
unc create-app my-app
unc create-app my-app -t api
unc create-app my-app -r username/my-templates
unc create-app my-app -b develop
```

**Process:**
1. Validates project name
2. Creates project directory
3. Downloads template from GitHub
4. Replaces placeholders
5. Initializes git repository
6. Checks for cargo-watch (installs if needed)

**Error Handling:**
If any step fails, the created directory is automatically removed.

### dev

Run the project with cargo-watch for hot reloading.

**Usage:**
```bash
unc dev
```

**What it does:**
1. Checks if cargo-watch is installed
2. Installs cargo-watch if not found
3. Runs `cargo watch -x run`

## Project Structure

```
src/
├── main.rs                # Entry point
├── cli.rs                 # CLI definitions
├── commands/
│   ├── mod.rs             # Command dispatcher
│   ├── create_app.rs      # CreateApp command
│   └── dev.rs             # Dev command
├── template/
│   ├── mod.rs             # Exports
│   ├── download.rs        # GitHub downloads
│   └── process.rs         # Placeholder replacement
└── utils/
    ├── mod.rs             # Exports
    ├── cargo.rs           # Cargo-watch utilities
    ├── file_ops.rs        # File operations
    ├── git.rs             # Git operations
    └── validation.rs      # Input validation
```

## Template Format

Templates should be organized as directories in a GitHub repository:

```
template-repo/
├── default/
│   ├── Cargo.toml
│   ├── src/
│   └── README.md
├── api/
│   └── ...
└── web/
    └── ...
```

**Placeholders:**
Use `{{project_name}}` in template files. It will be replaced with the actual project name.

Example:
```rust
fn main() {
    println!("Welcome to {{project_name}}!");
}
```

## Testing

Run all tests:
```bash
cargo test
```

Run quietly:
```bash
cargo test --quiet
```

Test specific module:
```bash
cargo test --lib utils
```

## Adding New Commands

1. Define in `cli.rs`:
```rust
pub enum Commands {
    CreateApp { ... },
    Dev,
    NewCommand { /* fields */ }
}
```

2. Create implementation file:
```rust
// src/commands/new_command.rs
use anyhow::Result;

pub fn new_command() -> Result<()> {
    // Implementation
    Ok(())
}
```

3. Add to dispatcher in `commands/mod.rs`:
```rust
pub mod new_command;

pub fn dispatch(command: Commands) -> Result<()> {
    match command {
        Commands::CreateApp { ... } => create_app(...),
        Commands::Dev => dev(),
        Commands::NewCommand { ... } => new_command(...),
    }
}
```

## Troubleshooting

**Directory already exists**
- Choose a different name or remove the existing directory

**Failed to download template**
- Check internet connection
- Verify repository exists and is public
- Check branch name

**Invalid project name**
- Must start with a letter
- Can only contain: letters, numbers, hyphens, underscores
- Valid: my-app, my_app, myApp
- Invalid: 123app, my app, @app

**Template not found**
- Verify template name exists in repository
- Templates must be directories in repository root

**cargo-watch installation failed**
- Check internet connection
- Install manually: `cargo install cargo-watch`

## Version

Current version: 0.1.0

Check version:
```bash
unc --version
```
