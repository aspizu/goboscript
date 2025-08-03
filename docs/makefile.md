# Makefile Documentation

The goboscript project includes a comprehensive Makefile to streamline development workflows. This Makefile provides convenient commands for building, testing, formatting, and managing the goboscript compiler.

## Prerequisites

- **Rust and Cargo**: Ensure you have Rust and Cargo installed. Visit [rustup.rs](https://rustup.rs/) for installation instructions.
- **Make**: The `make` command should be available on your system (usually pre-installed on Unix-like systems).

## Quick Start

```bash
# Show all available commands
make help

# Build the project
make build

# Run tests
make test

# Create a release build
make release

# Install goboscript globally
make install
```

## Available Commands

### Building

- **`make build`** - Build goboscript in debug mode
- **`make release`** - Build goboscript in release mode with optimizations
- **`make check`** - Check code without building (faster than build)

### Testing

- **`make test`** - Run all unit and integration tests
- **`make test-watch`** - Run tests continuously (requires `cargo-watch`)

### Code Quality

- **`make lint`** - Run Clippy linter to catch common mistakes
- **`make format`** - Format all code using rustfmt
- **`make format-check`** - Check if code is properly formatted (useful for CI)

### Installation

- **`make install`** - Install goboscript binary to your system
- **`make uninstall`** - Remove goboscript from your system

### Documentation

- **`make doc`** - Generate Rust documentation
- **`make doc-open`** - Generate documentation and open it in your browser

### Development Workflows

- **`make dev`** - Run format, lint, and test (recommended before committing)
- **`make quick`** - Build and test (quick development cycle)
- **`make full`** - Complete development cycle: clean, format, lint, test, doc, release

### CI/CD

- **`make ci-test`** - Run all checks suitable for continuous integration
- **`make ci-build`** - Clean build for deployment

### Utility

- **`make clean`** - Remove all build artifacts
- **`make example`** - Create an example goboscript project for testing
- **`make run ARGS="..."` - Run goboscript with custom arguments

## Usage Examples

### Basic Development Workflow

```bash
# Start working on goboscript
git clone https://github.com/aspizu/goboscript.git
cd goboscript

# Build and test
make build
make test

# Make your changes...

# Before committing
make dev
```

### Creating and Testing a New Project

```bash
# Install goboscript
make install

# Create an example project
make example

# Or manually create a new project
make run ARGS="new my-scratch-game"
```

### Release Preparation

```bash
# Full development cycle
make full

# Or step by step
make clean
make format
make lint
make test
make doc
make release
```

### Continuous Integration

```bash
# What CI should run
make ci-test
make ci-build
```

## Customization

The Makefile includes several variables at the top that can be customized:

```makefile
CARGO := cargo                    # Cargo command
TARGET_DIR := target             # Build output directory  
BINARY_NAME := goboscript        # Name of the binary
```

You can override these when running make:

```bash
# Use a different cargo command
make build CARGO="cross"

# Use a different target directory
make build TARGET_DIR="custom-target"
```

## Dependencies

Some commands require additional tools:

- **`cargo-watch`** (for `make test-watch`): Install with `cargo install cargo-watch`
- **`clippy`** (for `make lint`): Usually included with Rust installation

## Integration with Development Tools

### VS Code

Add this to your VS Code tasks.json:

```json
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "make dev",
            "type": "shell",
            "command": "make",
            "args": ["dev"],
            "group": "build",
            "presentation": {
                "echo": true,
                "reveal": "always"
            }
        }
    ]
}
```

### Git Hooks

Add to `.git/hooks/pre-commit`:

```bash
#!/bin/sh
make format-check && make lint && make test
```

## Troubleshooting

### Common Issues

1. **`make: command not found`**
   - Install make using your system package manager
   - On Windows, use WSL or install make via chocolatey

2. **`cargo-watch not found`**
   - Install with: `cargo install cargo-watch`

3. **Permission denied during install**
   - You may need sudo: `sudo make install`
   - Or install to user directory: `cargo install --path . --root ~/.local`

### Getting Help

- Run `make help` to see all available commands
- Check the [goboscript documentation](https://aspizu.github.io/goboscript/) for project-specific help
- Open an issue on GitHub for build problems