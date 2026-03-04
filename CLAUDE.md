# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

PJMAI is a Rust CLI tool for managing and switching between projects via shell aliases. It maintains a project registry at `~/.pjmai/config.toml` and integrates with bash/zsh through exit code signaling.

## Build Commands

```bash
cargo build              # Build the project
cargo check              # Check without building
cargo test               # Run tests
cargo test test_name     # Run a single test
cargo clippy --all-targets --all-features  # Run linter
```

## Architecture

**Shell Integration**: The `source-pjm.sh` script wraps the binary with a shell function that interprets exit codes:
- Exit 2: Change directory (`cd` to output path)
- Exit 3: Source file (`. output_path`)
- Exit 4: Error occurred (display error message)
- Other: Print output to console

This allows the CLI to affect the parent shell environment (which a subprocess normally cannot do).

**Module Structure** (all in `src/`):
- `args.rs` - CLI argument parsing via clap derive
- `command.rs` - Command implementations (add, change, list, remove, show, prompt, aliases, completions)
- `config.rs` - Configuration initialization and argument routing
- `projects.rs` - Data models (`ProjectsRegistry`, `ChangeToProject`, `Action`)
- `error.rs` - Custom error types (`PjmError`, `Result<T>`)
- `output.rs` - JSON output structures for `--json` flag
- `io.rs` - TOML file read/write operations
- `util.rs` - Path expansion, file checks

**Build Script**: `build.rs` generates version info with build timestamp at compile time.

## Code Requirements

The codebase enforces strict compiler settings via `#![deny(warnings, missing_docs)]`:
- All warnings are treated as errors
- All public items must have doc comments

## Testing

Integration tests use `PJMAI_CONFIG_DIR` environment variable to isolate test config from user config:

```bash
PJMAI_CONFIG_DIR=/tmp/test-pjmai cargo test    # Run with custom config dir
RUST_LOG=info pjmai -l list                     # Run with logging enabled
pjmai -d list                                   # Run with debug output
```

## Debug Mode

Use `-d` flag to print debug info before executing a command:

```bash
pjmai -d list
```

This outputs to stderr:
- Version info
- Environment variables (PJMAI_CONFIG_DIR, SHELL, HOME)
- Config directory and file paths
- Project list with current project marked

## Shell Aliases

After sourcing `source-pjm.sh` (or running `pjmai setup`):
- `adpj` - Add project
- `chpj` - Change to project
- `lspj` - List projects
- `rmpj` - Remove project
- `shpj` - Show current project
- `prpj` - Get project name for shell prompt
- `hlpj` - Show all aliases

## Installation

**Quick install** (from GitHub, once repo is public):
```bash
curl -fsSL https://raw.githubusercontent.com/wrightmikea/pjmai/main/install.sh | bash
```

**Local install** (for development):
```bash
./install.sh --local .
./install.sh --local . --prefix /usr/local/bin  # Custom prefix
./install.sh --local . --no-shell               # Skip shell config
```

The install script:
- Builds from source (requires git and cargo)
- Installs binary to `~/.local/bin/` (or custom prefix)
- Installs `source-pjm.sh` to `~/.pjmai/`
- Configures shell rc file (bash/zsh/fish)
- Installs shell completions

## Setup Command

Auto-configure shell integration (after manual binary install):

```bash
pjmai setup              # Auto-detect shell, install everything
pjmai setup zsh          # Specify shell explicitly
pjmai setup --shell-only # Only shell integration (source-pjm.sh)
pjmai setup --completions-only  # Only install completions
```

## JSON Output Mode

Use `--json` or `-j` flag for machine-readable output (useful for AI agents and scripting):

```bash
pjmai --json list          # List projects as JSON
pjmai --json show          # Show current project as JSON
pjmai --json change -p x   # Change outputs JSON (with error details if not found)
```

## Fast Tab Completion

The `complete` subcommand provides fast, native completion for shells:

```bash
pjmai complete projects          # List all project names
pjmai complete projects web      # List projects starting with "web"
pjmai complete commands          # List all command names
pjmai complete commands ch       # List commands starting with "ch"
```

The `source-pjm.sh` script uses this for dynamic tab completion on `chpj` and `rmpj`.

## Fuzzy Matching

The `change` command supports fuzzy matching: exact match, case-insensitive, prefix match, and substring match. Ambiguous matches show all candidates.
