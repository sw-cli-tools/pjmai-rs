# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

PJMAI-RS is a Rust CLI tool for managing and switching between projects via shell aliases. It maintains a project registry at `~/.pjmai/config.toml` and integrates with bash/zsh through exit code signaling.

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
- Exit 5: Execute environment setup (`eval` output script)
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
- `chpj` - Change to project (clears push/pop stack)
- `ctpj` - Show project context
- `evpj` - Manage project environment config
- `hlpj` - Show all aliases
- `hypj` - Show or jump to navigation history
- `lspj` - List projects
- `mvpj` - Rename project
- `popj` - Pop from project stack (shows destination)
- `prpj` - Get project name for shell prompt
- `pspj` - Push to stack and switch project
- `rmpj` - Remove project (supports `--all`)
- `scpj` - Scan for git repositories (supports `--reset`)
- `shpj` - Show current project
- `stpj` - Show or clear project stack
- `srcpj` - Source and approve .pjmai.sh

## Installation

**Quick install** (from GitHub, once repo is public):
```bash
curl -fsSL https://raw.githubusercontent.com/sw-cli-tools/pjmai-rs/main/install.sh | bash
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

## Development Update

For rapid development iteration, use the update script:

```bash
source update.sh
```

This script **must be sourced** (not executed) because it needs to reload shell integration in your current shell. It will:
- Build the release binary
- Copy `pjmai-rs` to `~/.local/bin/`
- Copy `source-pjm.sh` to `~/.pjmai/`
- Reload shell integration immediately

If you try to execute it directly (`./update.sh` or `sh update.sh`), it will show an error:
```
Error: Do not run this script, source it instead:
  source update.sh
```

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

## Non-Interactive Mode

Use `--yes` or `-y` flag for scripted/automated use (skips all confirmation prompts):

```bash
pjmai -y list              # Creates config without prompting if needed
pjmai -y scan ~/code       # Adds all found projects without confirmation
```

This is essential for VHS tape recordings and shell scripts.

## Scanning for Projects

Discover git repositories and add them as projects:

```bash
pjmai scan ~/code              # Scan with default depth (3)
pjmai scan ~/github --depth 4  # Deeper scan
pjmai scan ~/code --dry-run    # Preview without adding
pjmai scan --ignore tmp,cache  # Skip directories
```

Parses git remote origin URLs (including SSH aliases like `github.com-work`) to group by host/owner.

## Fast Tab Completion

The `complete` subcommand provides fast, native completion for shells:

```bash
pjmai complete projects          # List all project names (sorted by recency)
pjmai complete projects web      # List matching projects (prefix > segment > substring)
pjmai complete commands          # List all command names
pjmai complete commands ch       # List commands starting with "ch"
```

The `source-pjm.sh` script uses this for dynamic tab completion on `chpj`, `rmpj`, and `pspj`.

**Zsh completion** uses matchers for case-insensitive + substring matching:
- `chpj rank<TAB>` finds `sw-cl-rank-wav-rs` (substring match)
- `chpj SW<TAB>` finds `sw-*` projects (case-insensitive)

**Bash completion** uses prefix + case-insensitive matching via the Rust binary.

## Fuzzy Matching

The `change` command supports fuzzy matching: exact match, case-insensitive, prefix match, and substring match. Ambiguous matches show all candidates.

## Navigation History

Navigate to previously visited projects:

```bash
pjmai history            # Show numbered history (most recent last)
pjmai history 3          # Jump to entry #3 (like shell !nn)
```

## Stack Management

```bash
pjmai stack              # Show current stack (defaults to show)
pjmai stack clear        # Clear the stack (prompts for confirmation)
pjmai stack clear -y     # Clear without prompting
```

`chpj` clears the stack automatically (non-stack navigation abandons the push/pop workflow).

## Subdirectory Navigation

Navigate directly into subdirectories within a project with tab completion at each level:

```bash
chpj myproject<TAB>              # Complete project name
chpj myproject <TAB>             # Complete subdirs: src, tests, ...
chpj myproject src/<TAB>         # Complete nested: lib, main.rs parent dir, ...
chpj myproject src/lib<ENTER>    # cd to the subdir
```

Both space syntax and slash syntax work:
```bash
chpj myproject src lib           # Space-separated path parts
chpj myproject src/lib           # Slash-separated path
chpj myproject src/lib tests     # Mixed syntax
```

The `complete subdirs` subcommand provides subdir completion:
```bash
pjmai complete subdirs myproject           # List immediate subdirs
pjmai complete subdirs myproject src       # List subdirs of src/
pjmai complete subdirs myproject src/lib   # List subdirs of src/lib/
```

Errors are reported if the subdirectory doesn't exist or is a file:
```bash
chpj myproject nonexistent       # Error: subdirectory 'nonexistent' not found
chpj myproject README.md         # Error: 'README.md' is a file, not a directory
```
