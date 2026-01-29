# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

PJM1 is a Rust CLI tool for managing and switching between projects via shell aliases. It maintains a project registry at `~/.pjm/config.toml` and integrates with bash/zsh through exit code signaling.

## Build Commands

```bash
cargo build              # Build the project
cargo check              # Check without building
cargo test               # Run tests
cargo clippy --all-targets --all-features  # Run linter
```

## Architecture

**Shell Integration**: The `source-pjm.sh` script wraps the binary with a shell function that interprets exit codes:
- Exit 2: Change directory (`cd` to output path)
- Exit 3: Source file (`. output_path`)
- Other: Print output to console

This allows the CLI to affect the parent shell environment (which a subprocess normally cannot do).

**Module Structure**:
- `args.rs` - CLI argument parsing via structopt
- `command.rs` - Command implementations (add, change, list, remove, show, prompt, aliases)
- `config.rs` - Configuration initialization and argument routing
- `projects.rs` - Data models (`ProjectsRegistry`, `ChangeToProject`, `Action`)
- `io.rs` - TOML file read/write operations
- `util.rs` - Path expansion, file checks

**Build Script**: `build.rs` generates version info with build timestamp at compile time.

## Code Requirements

The codebase enforces strict compiler settings via `#![deny(warnings, missing_docs)]`:
- All warnings are treated as errors
- All public items must have doc comments

## Shell Aliases

After sourcing `source-pjm.sh`:
- `adpj` - Add project
- `chpj` - Change to project
- `lspj` - List projects
- `rmpj` - Remove project
- `shpj` - Show current project
- `prpj` - Get project name for shell prompt
- `hlpj` - Show all aliases

## Known Issue

The serde_derive dependency (1.0.104) is outdated and causes cfg-related warnings with recent Rust versions. Run `cargo update -p serde_derive` to update.
