# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

JiRust-CLI is a Rust-based command-line tool for interacting with Jira. It's structured as a workspace with two main components:
- `jirust-cli/` - The main CLI application
- `jira_v3_openapi/` - Auto-generated Jira API v3 bindings

The project supports both native compilation and WebAssembly (WASM) for Node.js usage.

## Development Commands

### Build and Test
```bash
# Build the main CLI
cargo build -p jirust-cli

# Build all components
cargo build

# Run tests for CLI
cargo test -p jirust-cli --all-features

# Run tests for API bindings
cargo test -p jira_v3_openapi --all-features

# Run all tests
cargo test --all-features
```

### Code Quality
```bash
# Run clippy (linting)
cargo clippy -p jirust-cli --all-features

# Format code
cargo fmt

# Check formatting
cargo fmt --check
```

### Installation and Usage
```bash
# Install CLI locally
cargo install --path jirust-cli

# Run CLI
jirust-cli --help

# Setup configuration
jirust-cli config setup
```

## Configuration

The CLI uses a TOML configuration file located at `~/.jirust-cli/jirust-cli.toml`. See `jirust-cli/config_example.toml` for structure:
- `[auth]` section with `auth_token`
- `[jira]` section with `jira_url`, resolution settings, and transition names

## Code Architecture

### Main Components
- **CLI Entry Point**: `jirust-cli/src/main.rs` - Platform-specific main functions (native vs WASM)
- **Library Interface**: `jirust-cli/src/lib.rs` - Core functions and WASM bindings
- **Command Structure**: `jirust-cli/src/args/` - Clap-based CLI argument definitions
- **Configuration**: `jirust-cli/src/config/` - Config file management
- **Executors**: `jirust-cli/src/executors/` - Business logic for each command type
- **Runners**: `jirust-cli/src/runners/` - Command execution orchestration
- **Utilities**: `jirust-cli/src/utils/` - Output formatting, JSON/table printing

### Command Categories
The CLI supports these main command categories:
- `config` - Configuration management
- `project` - Project operations (list, create)
- `version` - Version management (list, create, update, release, archive)
- `issue` - Issue operations (create, show, update, delete, search)
- `transition` - Issue state transitions
- `link` - Issue linking

### Key Design Patterns
- **Executor Pattern**: Each command type has a dedicated executor implementing `ExecJiraCommand` trait
- **Configuration Management**: Centralized config handling with automatic setup prompts
- **Multi-target Support**: Conditional compilation for native vs WASM targets
- **Output Flexibility**: JSON and table output formats with full/summary modes

## Development Notes

- The `jira_v3_openapi` crate is auto-generated from Jira's OpenAPI spec and should not be manually edited
- WASM support allows the CLI to run in Node.js environments
- Uses `prettytable-rs` for table output formatting
- Supports pagination for list operations
- Configuration is automatically prompted on first run if missing