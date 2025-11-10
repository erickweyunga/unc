# Wenzetu - Uncovr CLI

A command-line tool for Uncovr.

## Installation

```bash
cargo install wenzetu
```

## Quick Start

Create a new project:

```bash
wenzetu create-app my-app
cd my-app
cargo watch -x run
```

Run existing project with hot reload:

```bash
wenzetu dev
```

## Commands

### create-app

Create a new application from a template.

```bash
wenzetu create-app <name> [options]
```

**Options:**
- `-t, --template <name>` - Template to use (default: "default")
- `-r, --repo <repo>` - GitHub repository (default: "erickweyunga/uncovr-templates")
- `-b, --branch <branch>` - Branch to use (default: "main")
