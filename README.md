# Unc - Uncovr CLI

A command-line tool for Uncovr.

## Installation

```bash
cargo install unc
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

Run the project with cargo-watch for hot reloading. Optionally integrates with Tailwind CSS if configured.

```bash
unc dev
```

The `dev` command will automatically:
- Start cargo-watch for hot reloading
- Start Tailwind CSS watcher if enabled in `Cargo.toml` (requires Node.js/npx)

### help

Display help information.

```bash
unc help
```

## Tailwind CSS Integration

Unc can automatically run Tailwind CSS alongside your development server when using `unc dev`.

### Configuration

Add the following to your `Cargo.toml`:

```toml
[package.metadata.tailwind]
tw-input = ["src/styles/tailwind.css"]
tw-output = "public/output.css"
tw-watch-enabled = true
tw-watch-always = true
tw-optimize-minify = true
tw-optimize-map = false
```

### Configuration Options

- `tw-input` - Array of input CSS files (typically your main Tailwind CSS file)
- `tw-output` - Output path for the compiled CSS
- `tw-watch-enabled` - Enable/disable Tailwind CSS watcher (default: `false`)
- `tw-watch-always` - Keep watching even when stdin is closed (default: `false`)
- `tw-optimize-minify` - Minify the output CSS (default: `false`)
- `tw-optimize-map` - Generate source maps (default: `false`)

### Requirements

- Node.js and npm/npx installed
- Tailwind CSS will be automatically invoked via `npx tailwindcss`

### Output

When you run `unc dev`, you'll see clean, minimal output:

```
unc dev

  ▲ watching: cargo + tailwind
  ▲ ready in 500ms

  press ctrl+c to stop
```

When you press Ctrl+C:

```
  ▲ shutting down...
  ▲ stopped
```

### How It Works

When you run `unc dev` with Tailwind CSS enabled:

1. Unc reads the configuration from `[package.metadata.tailwind]` in your `Cargo.toml`
2. If `tw-watch-enabled = true`, it spawns a Tailwind CSS watcher process
3. The Tailwind CSS watcher runs alongside cargo-watch
4. Both processes watch for changes and rebuild automatically
5. When you stop the dev server (Ctrl+C), both processes are terminated **immediately and gracefully**

### Process Cleanup

Unc ensures that all spawned processes are properly cleaned up:

- **Ctrl+C handling**: When you press Ctrl+C, both cargo-watch and Tailwind CSS are killed instantly
- **Automatic cleanup**: If either process crashes or exits unexpectedly, the other is terminated automatically
- **Panic safety**: Even if the main process panics, all child processes are killed via Rust's `Drop` trait
- **Clean exit**: Minimal, professional output without verbose logging

### Disabling Tailwind CSS

To disable Tailwind CSS integration, either:
- Set `tw-watch-enabled = false` in your `Cargo.toml`
- Remove the `[package.metadata.tailwind]` section entirely

### Example Setup

```bash
# Create a new project
unc create-app my-app
cd my-app

# Add Tailwind CSS configuration to Cargo.toml
# (see Configuration section above)

# Create the input CSS file
mkdir -p src/styles
echo '@tailwind base; @tailwind components; @tailwind utilities;' > src/styles/tailwind.css

# Run the dev server with Tailwind CSS
unc dev
```

The output is clean and minimal - you'll see a simple status indicator showing what's running.
