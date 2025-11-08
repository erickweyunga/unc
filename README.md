# Unc - Uncovr CLI

A command-line tool for scaffolding Uncovr web applications.

## Installation

```bash
cargo install --path .
```

Or install from crates.io (once published):

```bash
cargo install unc
```

## Usage

### Create a New Application

```bash
# Create a new app with the default template
unc create-app my-app

# Create from a specific template
unc create-app my-app --template default

# Create from a custom repository
unc create-app my-app --repo yourusername/your-templates

# Use a specific branch
unc create-app my-app --branch develop
```

### Options

- `name` - Name of your application (required)
- `--template, -t` - Template to use (default: "default")
- `--repo, -r` - GitHub repository (default: "erickweyunga/uncovr-templates")
- `--branch, -b` - Git branch to use (default: "main")

## Templates

Templates are fetched from GitHub repositories. The default repository is:
`erickweyunga/uncovr-templates`

You can create your own template repository with the following structure:

```
your-repo/
├── template1/
│   ├── Cargo.toml
│   ├── src/
│   └── ...
├── template2/
│   ├── Cargo.toml
│   ├── src/
│   └── ...
└── README.md
```

### Placeholders

Templates can use the following placeholders that will be automatically replaced:

- `{{project_name}}` - The name of the project

## Examples

```bash
# Create a blog application
unc create-app my-blog

# Create an API server from a custom template
unc create-app my-api --template api-only

# Use a custom template repository
unc create-app my-app --repo johndoe/rust-templates --template full-stack
```

## Development

### Building

```bash
cargo build --release
```

### Testing

```bash
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT
