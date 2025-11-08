# Unc - Uncovr CLI

A command-line tool for scaffolding Uncovr web applications.

## Installation

```bash
cargo install --path .
```

## Quick Start

Create a new project:

```bash
unc create-app my-app
cd my-app
cargo watch -x run
```

Run existing project with hot reload:

```bash
unc dev
```

## Commands

### create-app

Create a new application from a template.

```bash
unc create-app <name> [options]
```

**Options:**
- `-t, --template <name>` - Template to use (default: "default")
- `-r, --repo <repo>` - GitHub repository (default: "erickweyunga/uncovr-templates")
- `-b, --branch <branch>` - Branch to use (default: "main")

**Examples:**

```bash
unc create-app my-app
unc create-app my-app -t api
unc create-app my-app -r username/my-templates -b develop
```

### dev

Run the project with cargo-watch for hot reloading.

```bash
unc dev
```

This command automatically installs cargo-watch if not present.

## Templates

Templates are fetched from GitHub repositories. The default repository is `erickweyunga/uncovr-templates`.

### Template Structure

```
repo/
├── default/
│   ├── Cargo.toml
│   ├── src/
│   └── ...
├── api/
│   ├── Cargo.toml
│   └── ...
```

### Placeholders

Use `{{project_name}}` in your template files. It will be replaced with the actual project name.

Example:
```rust
fn main() {
    println!("Welcome to {{project_name}}!");
}
```

## Project Name Rules

- Must start with a letter
- Can contain letters, numbers, hyphens, and underscores
- No spaces or special characters

Valid: `my-app`, `my_app`, `myApp`
Invalid: `123app`, `my app`, `@app`

## Development

Build:
```bash
cargo build --release
```

Test:
```bash
cargo test
```

Check code:
```bash
cargo check
```

## License

MIT