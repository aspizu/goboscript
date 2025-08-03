# Makefile for goboscript
# A text-based programming language that compiles to Scratch

# Variables
CARGO := cargo
TARGET_DIR := target
RELEASE_DIR := $(TARGET_DIR)/release
DEBUG_DIR := $(TARGET_DIR)/debug
BINARY_NAME := goboscript

# Default target
.PHONY: all
all: build

# Help target
.PHONY: help
help:
	@echo "goboscript Makefile"
	@echo ""
	@echo "Available targets:"
	@echo "  help        Show this help message"
	@echo "  build       Build the project in debug mode"
	@echo "  release     Build the project in release mode"
	@echo "  test        Run all tests"
	@echo "  test-watch  Run tests in watch mode"
	@echo "  lint        Run clippy linter"
	@echo "  format      Format code with rustfmt"
	@echo "  format-check Check code formatting"
	@echo "  clean       Clean build artifacts"
	@echo "  install     Install goboscript binary"
	@echo "  uninstall   Uninstall goboscript binary"
	@echo "  doc         Generate documentation"
	@echo "  doc-open    Generate and open documentation"
	@echo "  check       Check code without building"
	@echo "  example     Create a new goboscript example project"
	@echo "  run         Run goboscript (use ARGS for arguments)"
	@echo ""
	@echo "Examples:"
	@echo "  make build"
	@echo "  make release"
	@echo "  make run ARGS='new my-project'"
	@echo "  make test"

# Build targets
.PHONY: build
build:
	@echo "Building goboscript in debug mode..."
	$(CARGO) build

.PHONY: release
release:
	@echo "Building goboscript in release mode..."
	$(CARGO) build --release

# Test targets
.PHONY: test
test:
	@echo "Running tests..."
	$(CARGO) test

.PHONY: test-watch
test-watch:
	@echo "Running tests in watch mode..."
	$(CARGO) watch -x test

# Linting and formatting
.PHONY: lint
lint:
	@echo "Running clippy..."
	$(CARGO) clippy -- -D warnings

.PHONY: format
format:
	@echo "Formatting code..."
	$(CARGO) fmt

.PHONY: format-check
format-check:
	@echo "Checking code formatting..."
	$(CARGO) fmt -- --check

# Cleaning
.PHONY: clean
clean:
	@echo "Cleaning build artifacts..."
	$(CARGO) clean

# Installation
.PHONY: install
install: release
	@echo "Installing goboscript..."
	$(CARGO) install --path .

.PHONY: uninstall
uninstall:
	@echo "Uninstalling goboscript..."
	$(CARGO) uninstall $(BINARY_NAME)

# Documentation
.PHONY: doc
doc:
	@echo "Generating Rust documentation..."
	$(CARGO) doc

.PHONY: doc-open
doc-open:
	@echo "Generating and opening Rust documentation..."
	$(CARGO) doc --open

.PHONY: docs-serve
docs-serve:
	@echo "Serving MkDocs documentation locally..."
	mkdocs serve

.PHONY: docs-build
docs-build:
	@echo "Building MkDocs documentation..."
	mkdocs build

.PHONY: docs-deploy
docs-deploy:
	@echo "Deploying MkDocs documentation..."
	mkdocs gh-deploy

# Code checking
.PHONY: check
check:
	@echo "Checking code..."
	$(CARGO) check

# Development helpers
.PHONY: run
run: build
	@echo "Running goboscript..."
	$(CARGO) run -- $(ARGS)

.PHONY: example
example: install
	@echo "Creating example goboscript project..."
	@mkdir -p examples/demo
	@cd examples/demo && goboscript new demo-project
	@echo "Example project created in examples/demo/demo-project"

# CI/CD helpers
.PHONY: ci-test
ci-test: format-check lint test
	@echo "All CI checks passed!"

.PHONY: ci-build
ci-build: clean release
	@echo "CI build completed!"

# Development workflow
.PHONY: dev
dev: format lint test
	@echo "Development checks completed!"

# Quick development cycle
.PHONY: quick
quick: build test
	@echo "Quick development cycle completed!"

# Full development cycle
.PHONY: full
full: clean format lint test doc release
	@echo "Full development cycle completed!"