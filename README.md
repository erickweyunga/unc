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

Run the project with cargo-watch for hot reloading.

```bash
unc dev
```

### help

Display help information.

```bash
unc help
```
