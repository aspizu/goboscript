# Development Guide

This guide covers everything you need to know to contribute to goboscript development.

## Prerequisites

Before you start developing goboscript, make sure you have:

- **Rust** (latest stable version recommended) - Install from [rustup.rs](https://rustup.rs/)
- **Git** - For version control
- **Make** (optional but recommended) - For streamlined development workflow
- **A text editor or IDE** - VS Code with rust-analyzer is recommended

## Getting Started

### 1. Clone the Repository

```bash
git clone https://github.com/aspizu/goboscript.git
cd goboscript
```

### 2. Build the Project

#### Using Make (Recommended)

```bash
# Show all available commands
make help

# Build in debug mode
make build

# Build optimized release version
make release
```

#### Using Cargo Directly

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### 3. Run Tests

```bash
# Using Make
make test

# Using Cargo
cargo test
```

### 4. Install for Development

```bash
# Install goboscript globally
make install

# Or with Cargo
cargo install --path .
```

## Development Workflow

### Daily Development with Make

goboscript includes a comprehensive Makefile that simplifies common development tasks:

```bash
# Quick development cycle (recommended)
make dev          # Format code, run linter, run tests

# Individual commands
make build        # Debug build
make test         # Run all tests
make format       # Format code with rustfmt
make lint         # Run clippy linter
make clean        # Clean build artifacts
```

### Development Targets

- **`make quick`** - Fast build and test cycle
- **`make dev`** - Complete pre-commit checks (format + lint + test)
- **`make full`** - Full development pipeline (clean + format + lint + test + doc + release)

### Testing Your Changes

```bash
# Run all tests
make test

# Test with a real project
make run ARGS="new test-project"
cd test-project
make run ARGS="compile main.gobo"

# Create example for testing
make example
```

### Code Quality

Before submitting changes, ensure code quality:

```bash
# Format code
make format

# Run linter
make lint

# Check formatting (useful for CI)
make format-check

# Run all quality checks
make dev
```

## Making Changes

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-description
```

### 2. Development Cycle

```bash
# Make your changes...

# Test your changes
make dev

# Commit your changes
git add .
git commit -m "Add feature: description"
```

### 3. Before Submitting

```bash
# Run complete development cycle
make full

# Ensure everything works
make clean
make build
make test
make install

# Test the installed version
goboscript new test-project
goboscript --help
```

## Testing

### Running Tests

```bash
# All tests
make test

# With output
cargo test -- --nocapture

# Specific test
cargo test test_name

# Integration tests only
cargo test --test integration_tests
```

### Writing Tests

- **Unit tests**: Add `#[cfg(test)]` modules in source files
- **Integration tests**: Add files in `tests/` directory
- **Example tests**: Test compilation of example projects

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_basic() {
        // Test your parser functionality
    }
}
```

## Documentation

### Building Documentation

```bash
# Generate Rust docs
make doc

# Generate and open docs
make doc-open

# Using Cargo
cargo doc --open
```

### Documentation Standards

- Add doc comments (`///`) to all public functions
- Include examples in doc comments
- Update this guide when adding new features
- Update the main README.md for user-facing changes

## Debugging

### Debug Builds

```bash
# Build with debug info
make build

# Run with debug output
RUST_LOG=debug cargo run -- compile example.gobo
```

### Using a Debugger

With VS Code and rust-analyzer:
1. Set breakpoints in your code
2. Use "Run and Debug" panel
3. Select "Debug unit tests" or create custom launch configuration

### Logging

goboscript uses the `log` crate for logging:

```rust
use log::{debug, info, warn, error};

debug!("Parsing file: {}", filename);
info!("Compilation successful");
warn!("Deprecated syntax used");
error!("Compilation failed: {}", error);
```

## Performance

### Benchmarking

```bash
# If benchmarks exist
cargo bench

# Profile with release build
cargo build --release
time target/release/goboscript compile large-project.gobo
```

### Optimization Tips

- Use `cargo build --release` for performance testing
- Profile with tools like `perf` or `flamegraph`
- Benchmark before and after changes

## Contributing Guidelines

### Code Style

- Follow Rust standard formatting (enforced by `make format`)
- Use meaningful variable and function names
- Add comments for complex logic
- Keep functions focused and small

### Commit Messages

Use conventional commit format:

```
feat: add support for new Scratch blocks
fix: resolve parser error with nested loops  
docs: update installation instructions
test: add integration tests for compiler
refactor: simplify AST structure
```

### Pull Request Process

1. **Fork the repository**
2. **Create a feature branch**
3. **Make your changes**
4. **Run `make dev` to ensure quality**
5. **Write/update tests**
6. **Update documentation**
7. **Submit pull request**

### Pull Request Checklist

- [ ] Code follows project style guidelines
- [ ] All tests pass (`make test`)
- [ ] Code is properly formatted (`make format`)
- [ ] Linter passes (`make lint`)
- [ ] Documentation is updated if needed
- [ ] Commit messages follow conventional format
- [ ] PR description explains the changes clearly

## Common Development Tasks

### Adding a New Scratch Block

1. Add block definition to the parser
2. Update the AST structure
3. Implement code generation
4. Add tests for the new block
5. Update documentation

### Fixing a Bug

1. Create a test that reproduces the bug
2. Fix the issue
3. Ensure the test now passes
4. Run full test suite

### Adding a Language Feature

1. Design the syntax
2. Update the parser
3. Modify the AST
4. Implement code generation
5. Add comprehensive tests
6. Update language documentation

## Troubleshooting

### Common Issues

**Build Failures:**
```bash
# Clean and rebuild
make clean
make build
```

**Test Failures:**
```bash
# Run specific failing test with output
cargo test failing_test_name -- --nocapture
```

**Clippy Warnings:**
```bash
# Fix automatically where possible
cargo clippy --fix
```

**Formatting Issues:**
```bash
# Format all code
make format
```

### Getting Help

- Check existing [GitHub issues](https://github.com/aspizu/goboscript/issues)
- Join discussions in GitHub Discussions
- Read the [main documentation](https://aspizu.github.io/goboscript/)
- Ask questions in new GitHub issues

## Resources

- [Rust Book](https://doc.rust-lang.org/book/) - Learn Rust
- [Cargo Book](https://doc.rust-lang.org/cargo/) - Rust's package manager
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/) - Linter documentation
- [Scratch File Format](https://en.scratch-wiki.info/wiki/Scratch_File_Format) - Understanding Scratch projects

For detailed Makefile documentation, see the [Makefile Guide](makefile.md).