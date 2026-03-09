# PJMAI-RS - Project Management CLI

A Rust CLI tool for managing and quickly switching between projects via shell aliases.

## Overview

PJMAI-RS helps developers manage multiple projects by maintaining a registry of project directories and files. It integrates with your shell to enable quick project switching with short commands, and supports per-project environment configuration.

## Resources

- [Introducing PJMAI-RS](https://software-wrighter-lab.github.io/2026/03/05/tbt-pjmai-rs-project-manager/) — blog post with overview and walkthrough
- [Navigation History & Fuzzy Completion](https://software-wrighter-lab.github.io/2026/03/07/pjmai-rs-navigation-history-and-fuzzy-completion/) — blog post on latest features
- [Demo Video](https://www.youtube.com/watch?v=4eWyhWjF3dg) — see pjmai-rs in action
- [Discord](https://discord.com/invite/Ctzk5uHggZ) — community chat and support

## Installation

### Quick Install (Recommended)

Install pjmai with a single command:

```bash
curl -fsSL https://raw.githubusercontent.com/sw-cli-tools/pjmai-rs/main/install.sh | bash
```

This will:
- Clone and build from source (requires git and Rust)
- Install the binary to `~/.local/bin/`
- Install shell integration script
- Configure your shell (bash, zsh, or fish)
- Install shell completions

Options:
```bash
# Install to a custom location
curl -fsSL https://raw.githubusercontent.com/sw-cli-tools/pjmai-rs/main/install.sh | bash -s -- --prefix /usr/local/bin

# Skip shell integration (configure manually later)
curl -fsSL https://raw.githubusercontent.com/sw-cli-tools/pjmai-rs/main/install.sh | bash -s -- --no-shell

# Skip completions
curl -fsSL https://raw.githubusercontent.com/sw-cli-tools/pjmai-rs/main/install.sh | bash -s -- --no-completions
```

### From Source (Manual)

```bash
# Clone the repository
git clone https://github.com/sw-cli-tools/pjmai-rs.git
cd pjmai

# Build the binary
cargo build --release

# Copy to your PATH (create ~/.local/bin if needed)
mkdir -p ~/.local/bin
cp target/release/pjmai ~/.local/bin/

# Run setup to configure your shell
pjmai setup
```

### Uninstallation

```bash
# Preview what would be removed
./uninstall.sh --dry-run

# Full uninstall (prompts before removing projects)
./uninstall.sh

# Uninstall but keep your project list for later reinstall
./uninstall.sh --keep-config
```

The uninstall script:
- Removes the binary from `~/.local/bin/`
- Removes shell completions
- Cleans PJMAI entries from `~/.zshrc` (creates backup first)
- Removes `~/.pjmai/` directory (unless `--keep-config`)

### Shell Integration (Required for Manual Install)

**Option 1: Automatic Setup (Recommended)**

Run the setup command to automatically configure your shell:

```bash
pjmai setup
```

This will:
- Auto-detect your shell (bash, zsh, fish)
- Add shell integration to your rc file (~/.bashrc, ~/.zshrc, etc.)
- Install shell completions

You can also specify options:
```bash
pjmai setup zsh              # Specify shell explicitly
pjmai setup --shell-only     # Only add shell integration
pjmai setup --completions-only  # Only install completions
```

**Option 2: Manual Setup**

Add the following to your shell's rc file:

**For Zsh (~/.zshrc) on macOS:**
```bash
# PJMAI project management
export PATH="$HOME/.local/bin:$PATH"
source /path/to/pjmai/source-pjm.sh
```

**For Bash (~/.bashrc) on Linux:**
```bash
# PJMAI project management
export PATH="$HOME/.local/bin:$PATH"
source /path/to/pjmai/source-pjm.sh
```

Then reload your shell: `source ~/.zshrc` (or `~/.bashrc`)

This sets up:
- Shell aliases (`adpj`, `chpj`, `lspj`, etc.)
- A wrapper function that allows PJMAI to change your working directory

## Quick Start

```bash
# Add your first project (config created automatically on first use)
adpj myproject -f ~/code/myproject

# List all projects
lspj

# Switch to a project (changes directory)
chpj myproject

# Show current project
shpj

# Get project name (for shell prompts)
prpj
```

## Commands

| Command | Alias | Description |
|---------|-------|-------------|
| `pjmai add -p <name> -f <path>` | `adpj` | Add a new project |
| `pjmai change -p <name> [subdir...]` | `chpj` | Switch to a project (clears stack; `--push` to push instead) |
| `pjmai history [N]` | `hypj` | Show or jump to navigation history |
| `pjmai list` | `lspj` | List all projects |
| `pjmai push -p <name>` | `pspj` | Push current to stack, switch to project |
| `pjmai pop` | `popj` | Pop from stack, return to previous project |
| `pjmai remove -p <name>` | `rmpj` | Remove a project (`--all` to remove all) |
| `pjmai rename -f <old> -t <new>` | `mvpj` | Rename a project |
| `pjmai show` | `shpj` | Show current project (and stack) |
| `pjmai stack [show\|clear]` | `stpj` | Show or clear the project stack |
| `pjmai prompt` | `prpj` | Output current project name (for prompts) |
| `pjmai aliases` | `hlpj` | Show all available aliases |
| `pjmai complete projects [prefix]` | - | Fuzzy project name completion for shells |
| `pjmai complete subdirs <project> [path...]` | - | Fast subdirectory completion for shells |
| `pjmai complete commands [prefix]` | - | Fast command name completion for shells |
| `pjmai completions <shell>` | - | Generate shell completions |
| `pjmai edit -p <name> [options]` | `edpj` | Edit project properties (description, language, pin) |
| `pjmai scan [dir]` | `scpj` | Scan for git repos (`--reset` for fresh scan) |
| `pjmai context [-p project]` | `ctpj` | Show project context for AI agents |
| `pjmai env -p <name> <action>` | `evpj` | Manage project environment config |
| `pjmai query -p <name>` | `qypj` | Check if a project exists (exits 0/1) |
| `pjmai exports [--format FMT]` | `xppj` | Export paths as shell named directories |
| `pjmai config export` | - | Export configuration to stdout |
| `pjmai config import <file>` | - | Import configuration from a file |
| `pjmai setup [shell]` | - | Auto-configure shell integration |
| - | `srcpj` | Source and approve .pjmai.sh in current directory |

### Global Flags

| Flag | Short | Description |
|------|-------|-------------|
| `--json` | `-j` | Output in JSON format for machine parsing |
| `--logging` | `-l` | Enable logging (requires `RUST_LOG=info`) |
| `--debug` | `-d` | Print debug info (config paths, projects) before command |
| `--yes` | `-y` | Assume yes to all prompts (for scripts/automation) |

## Scanning for Projects

Automatically discover git repositories and add them as projects:

```bash
# Scan home directory (default depth: 3)
scpj

# Scan a specific directory
scpj ~/code --depth 4

# Preview what would be found without adding
scpj ~/github --dry-run

# Skip specific directories
scpj ~/code --ignore vendor,dist,tmp

# Add all without confirmation
scpj ~/projects --add-all
```

```bash
# Fresh re-scan (preserves metadata: descriptions, tags, notes, env config)
scpj --reset -y ~/github

# Non-interactive scan
scpj ~/code -y
```

The scan command:
- Finds directories containing `.git`
- Parses git remote origin to extract owner/organization
- Groups repositories by host/owner for display
- Suggests nicknames based on repo name (handles collisions)
- Honors `.gitignore` patterns when recursing
- Skips already-registered projects (by path)
- Auto-detects programming languages (e.g., `rust`, `python`, `rust+python` for polyglot projects)
- `--reset` preserves user metadata (descriptions, tags, notes, last_used, env config)
- Creates timestamped backups before destructive operations

## JSON Output Mode

Use the `--json` flag to get machine-readable JSON output, useful for scripting and AI agent integration:

```bash
# List projects as JSON
pjmai --json list
{
  "projects": [
    {
      "name": "webapp",
      "path": "/home/user/code/webapp",
      "type": "directory",
      "is_current": true
    }
  ],
  "current_project": "webapp",
  "total": 1
}

# Show current project as JSON
pjmai --json show
{
  "name": "webapp",
  "path": "/home/user/code/webapp",
  "type": "directory"
}

# Errors include suggestions in JSON
pjmai --json change -p notfound
{
  "code": "PROJECT_NOT_FOUND",
  "message": "Project 'notfound' not found",
  "similar_projects": ["webapp", "webapi"],
  "hint": "Use 'pjmai list' to see all projects"
}
```

## Shell Completions

Generate and install shell completions for tab-completion support:

```bash
# Bash (add to ~/.bashrc)
pjmai completions bash > ~/.local/share/bash-completion/completions/pjmai

# Zsh (add to ~/.zshrc or a file in your fpath)
pjmai completions zsh > ~/.zsh/completions/_pjmai

# Fish
pjmai completions fish > ~/.config/fish/completions/pjmai.fish

# PowerShell
pjmai completions powershell >> $PROFILE
```

## Usage Scenarios

### Setting Up a New Project

When starting work on a new project, add it to PJMAI:

```bash
# Add a directory-based project
adpj -p webapp -f ~/code/my-webapp

# Add a file-based project (sources the file when switching)
adpj -p devenv -f ~/envs/dev-environment.sh
```

### Switching Between Projects

Quickly switch context between projects:

```bash
# Switch to the webapp project (changes directory)
chpj webapp

# Switch to devenv (sources the environment file)
chpj devenv

# Fuzzy matching: partial names work too
chpj web      # matches "webapp" if unique
chpj WEBAPP   # case-insensitive matching
```

The `change` command supports fuzzy matching:
- **Exact match**: `webapp` matches project named "webapp"
- **Case-insensitive**: `WEBAPP` matches "webapp"
- **Prefix match**: `web` matches "webapp" (if unique)
- **Substring match**: `app` matches "webapp" (if unique)
- **Ambiguous**: If multiple projects match, shows all matches

### Subdirectory Navigation

Navigate directly into subdirectories within a project with tab completion:

```bash
# Tab completion works at each level
chpj myproject<TAB>              # Complete project name
chpj myproject <TAB>             # Complete subdirs: src, tests, docs...
chpj myproject src/<TAB>         # Complete nested dirs: lib, bin...
chpj myproject src/lib<ENTER>    # cd to ~/code/myproject/src/lib
```

Both space and slash syntax work:
```bash
chpj myproject src lib           # Space-separated path parts
chpj myproject src/lib           # Slash-separated path
chpj myproject src/lib tests     # Mixed syntax
```

Helpful error messages when paths don't exist:
```bash
chpj myproject nonexistent
# Error: subdirectory 'nonexistent' not found in project 'myproject'

chpj myproject README.md
# Error: 'README.md' is a file, not a directory
```

JSON output includes the subdir:
```bash
pjmai --json change -p myproject src/lib
{
  "name": "myproject",
  "path": "/home/user/code/myproject/src/lib",
  "type": "directory",
  "action": "cd",
  "subdir": "src/lib"
}

### Push/Pop Stack Navigation

Use push/pop for temporary project switches when you need to return:

```bash
# Working in webapp, need to check something in api
pspj api           # Push webapp to stack, switch to api
# — or equivalently —
chpj --push api    # Same thing: push + switch (with chpj features like subdirs)

# Need to check config too
pspj config        # Push api to stack, switch to config

# Done with config, return to api
popj               # Pop from stack, back to api

# Done with api, return to webapp
popj               # Pop from stack, back to webapp

# Stack is empty now
popj               # Warning: stack empty, staying in webapp
```

View the current stack:
```bash
shpj
# Output:
# >config   ~/code/config
#  Stack (2): api <- webapp
```

The stack persists across terminal sessions and is stored in your config.

### Shell Prompt Integration

Add the current project to your shell prompt automatically:

```bash
pjmai setup --prompt
```

Or manually add to your `.bashrc` or `.zshrc`:

```bash
_pjm_prompt() {
  local proj=$(prpj 2>/dev/null)
  [[ -n "$proj" ]] && echo "[$proj] "
}
PS1='$(_pjm_prompt)\w \$ '
```

The prompt shows the current project and stack depth:

```
[webapp] ~/code $          # Current project, no stack
[api:1] ~/code $           # In api, 1 item on stack (webapp)
[config:2] ~/code $        # In config, 2 items on stack
~/code $                   # No project set
```

The number after `:` indicates how many `popj` commands will return you to previous projects.

### Project Environment Files (.pjmai.sh)

Set up per-project environment variables, activate virtual environments, or run setup commands.

**Create a `.pjmai.sh` in your project:**
```bash
# ~/code/myproject/.pjmai.sh
source .venv/bin/activate
export SRCROOT="$PWD"
export PYTHONPATH="$PWD/src"
```

**Security model (approve once, auto-source if unchanged):**

```bash
chpj myproject
# Output: Found .pjmai.sh - inspect: 'cat .pjmai.sh', approve: 'srcpj'

# First time: inspect the file
cat .pjmai.sh

# If trusted, approve and source it
srcpj
# Output: Sourcing .pjmai.sh...
# Output: Approved - will auto-source until file changes

# Future visits: auto-sourced silently (hash unchanged)
chpj other-project
chpj myproject      # .pjmai.sh sourced automatically
```

**How it works:**
- First visit: Warning shown, manual `srcpj` required
- `srcpj`: Sources the file AND saves a hash approval
- Future visits: If file hash matches approval, auto-sources silently
- File changes: Warning shown again, re-approval required

This prevents untrusted code execution from cloned repos while enabling convenient environment setup for your own projects.

### Per-Project Environment Configuration

Configure environment variables, PATH modifications, and entry/exit hooks for individual projects:

```bash
# Set environment variable for a project
evpj webapp set DATABASE_URL "postgres://localhost/webapp"

# Prepend paths to PATH (for project-local binaries)
evpj webapp path-prepend "./.venv/bin"
evpj webapp path-prepend "./node_modules/.bin"

# Add command to run when entering project
evpj webapp on-enter "source .venv/bin/activate"

# Add command to run when LEAVING project (cleanup)
evpj webapp on-exit "deactivate"

# View project environment config
evpj webapp show
# Output:
# Environment config for project webapp:
#   Variables:
#     DATABASE_URL=postgres://localhost/webapp
#   Path prepend:
#     ./.venv/bin
#     ./node_modules/.bin
#   On enter:
#     source .venv/bin/activate
#   On exit:
#     deactivate

# Remove a path from the prepend list
evpj webapp path-remove "./node_modules/.bin"

# Clear all environment config
evpj webapp clear
```

When you switch to a project with environment config (`chpj webapp`), the shell will:
1. Run any on_exit commands from the previous project (cleanup)
2. Change to the new project directory
3. Prepend paths to PATH
4. Export environment variables
5. Store on_exit commands for later (when switching away)
6. Run on_enter commands
7. Check for `.pjmai.sh` (as usual)

The environment config is stored in `~/.pjmai/config.toml`:

```toml
[[project]]
name = "webapp"
[project.action]
file_or_dir = "~/code/webapp"
[project.metadata.environment]
vars = { DATABASE_URL = "postgres://localhost/webapp" }
path_prepend = ["./.venv/bin", "./node_modules/.bin"]
on_enter = ["source .venv/bin/activate"]
on_exit = ["deactivate"]
```

### Auto-Detecting Environment Configuration

Instead of manually configuring each feature, use `auto-detect` to scan your project and configure automatically:

```bash
# Preview what would be configured (dry run)
evpj webapp auto-detect --dry-run

# Auto-detect and apply configuration
evpj webapp auto-detect
```

Auto-detect recognizes:
- **Python venv** (`.venv/` or `venv/`): Adds activate/deactivate commands and PATH
- **Node.js** (`.nvmrc`): Adds `nvm use` command
- **Node modules** (`node_modules/.bin/`): Adds to PATH
- **Rust** (`Cargo.toml`): Adds `./target/debug` to PATH
- **direnv** (`.envrc`): Suggests sourcing (with security notice)
- **Python project** (`pyproject.toml` without venv): Suggests creating venv

Example output:
```
Detected environment features for project webapp:
  python-venv (from .venv/)
    Path prepend: ./.venv/bin
    On enter: source .venv/bin/activate
    On exit: deactivate
  node-nvm (from .nvmrc)
    On enter: nvm use

Configuration applied.
```

### Cross-Project File Operations (Named Directories)

Export project paths so you can reference them in `cp`, `mv`, `ls`, and other commands:

```bash
# Zsh: enable ~nickname/path syntax (add to .zshrc)
eval "$(xppj)"

# Now use ~nickname/path with tab completion at every level:
ls ~webapp/src/
cp ~webapp/README.md ~backend/
vim ~mylib/src/main.rs

# Bash/Fish alternatives (use $PJMAI_* env vars):
eval "$(xppj --format bash)"    # export PJMAI_WEBAPP="/home/user/code/webapp"
eval "$(xppj --format fish)"    # set -gx PJMAI_WEBAPP "/home/user/code/webapp"
```

### Pinned Projects (Survive `scan --reset`)

Pin custom projects (e.g., Dropbox directories) so they're re-added after `scpj --reset`:

```bash
# Add with --pinned flag (auto-appends to ~/.pjmai/pinned.sh)
adpj shared-images -f ~/Dropbox/shared/images --pinned
adpj shared-docs -f ~/Dropbox/shared/docs --pinned

# Or manually create ~/.pjmai/pinned.sh:
qypj shared-images 2>/dev/null || adpj shared-images -f ~/Dropbox/shared/images
qypj shared-docs   2>/dev/null || adpj shared-docs   -f ~/Dropbox/shared/docs

# After scan --reset, pinned.sh is auto-sourced
scpj --reset ~/github    # Clears all, re-scans, then re-adds pinned projects
```

### Querying Projects

Check if a project exists (useful in scripts):

```bash
# Exit code 0 if found, 1 if not
qypj webapp && echo "exists"

# Conditional add
qypj shared-images 2>/dev/null || adpj shared-images -f ~/Dropbox/shared/images
```

### Managing Your Project List

```bash
# View all projects (current project highlighted)
lspj

# Example output:
# >webapp   ~/code/my-webapp
#  backend  ~/code/backend-api
#  devenv   ~/envs/dev-environment.sh

# Extended info (language, description, tags)
lspj --long

# Filter by language
lspj --lang rust

# Sort by recently used (tracks chpj/pspj/popj)
lspj --recent

# Sort by filesystem modification time
lspj --modified

# Remove a project you no longer need
rmpj -p oldproject
```

### Editing Project Properties (edpj)

Update metadata for an existing project:

```bash
# Set description and language
edpj webapp -D "Customer portal" -L typescript

# Set group
edpj webapp -g work

# Pin a project (survives scan --reset)
edpj webapp --pin

# Unpin a project
edpj webapp --unpin
```

## Configuration

PJMAI stores its configuration at `~/.pjmai/config.toml`. The file is created automatically on first use.

### Custom Configuration Directory

Set the `PJMAI_CONFIG_DIR` environment variable to use a different configuration directory:

```bash
export PJMAI_CONFIG_DIR=~/my-custom-config
```

This is useful for testing or maintaining separate project registries.

### Export and Import

Backup your configuration or share it across machines:

```bash
# Export to TOML (default)
pjmai config export > pjmai-backup.toml

# Export to JSON
pjmai config export --format json > pjmai-backup.json

# Import from a file
pjmai config import pjmai-backup.toml

# Preview import without making changes
pjmai config import --dry-run colleague-config.toml

# Merge with existing config (updates metadata for existing projects)
pjmai config import --merge shared-config.toml
```

Import behavior:
- **Default**: Adds new projects, skips existing ones (by name)
- **With `--merge`**: Adds new projects, updates metadata for existing ones
- **With `--dry-run`**: Shows what would happen without making changes

## How It Works

PJMAI-RS uses exit codes to communicate with the shell wrapper script:

- **Exit 2**: The output is a directory path; the shell will `cd` to it
- **Exit 3**: The output is a file path; the shell will `source` it
- **Exit 4**: Error occurred; the shell displays the error message
- **Exit 5**: The output is a shell script; the shell will `eval` it (environment setup)

This mechanism allows the CLI (which runs as a subprocess) to affect the parent shell's environment.

## Development

### Building

```bash
cargo build
```

### Quick Update (Development)

For rapid development iteration, use the update script:

```bash
source update.sh
```

This script **must be sourced** (not executed). It will:
- Build the release binary (`cargo build --release`)
- Install `pjmai-rs` to `~/.local/bin/`
- Install `source-pjm.sh` to `~/.pjmai/`
- Reload shell integration in your current shell

The script enforces sourcing - if you try to execute it directly, you'll get an error:
```
Error: Do not run this script, source it instead:
  source update.sh
```

Optional: install to a custom prefix:
```bash
source update.sh --prefix /usr/local/bin
```

### Testing

```bash
cargo test
```

### Linting

```bash
cargo clippy --all-targets --all-features
```

### Running with Logging

```bash
RUST_LOG=info pjmai -l list
```

## Non-Interactive Mode

The `--yes` (or `-y`) flag enables non-interactive mode for scripting and automation:

```bash
# Create config file without prompting
pjmai -y list

# Scan and add all projects without confirmation
pjmai -y scan ~/code

# Equivalent to: pjmai scan ~/code --add-all
```

This is useful for:
- **Shell scripts**: Automated setup or CI workflows
- **VHS tape recordings**: Terminal demos that can't handle interactive prompts
- **Batch operations**: Scripted project management

The flag affects:
- Config file creation prompt (`Create config file? [Y/n]`)
- Scan confirmation prompt (`Add all N project(s)? [Y/n]`)

## Demo Recordings

The `demo/` directory contains VHS tape files for creating terminal recordings:

```bash
# Install VHS (macOS)
brew install vhs

# Setup demo environment
./demo/setup-demo.sh

# Record a demo
vhs demo/full-workflow.tape
```

Available demos:
- `full-workflow.tape` - Complete walkthrough: installation, adding, switching, and removing projects
- `basic-workflow.tape` - Core commands: add, list, show, prompt
- `project-management.tape` - Adding and removing projects
- `error-handling.tape` - Error messages for invalid operations
- `scan-workflow.tape` - Scanning for git repos, renaming, and removing projects
- `prompt-integration.tape` - Shell prompt with push/pop stack navigation
- `env-approval.tape` - Secure .pjmai.sh environment file approval workflow

## Historical Context

The core idea — giving projects nicknames and switching fast — dates back to the 1980s. The concept evolved through several eras, from academic software project management systems to modern Rust CLI tooling with AI agent support.

For the full story, see the [Historical Context](https://software-wrighter-lab.github.io/2026/03/05/tbt-pjmai-rs-project-manager/#historical-context) section of the introductory blog post.

### References

| Era | Resource | Description |
|-----|----------|-------------|
| 1980s | [4.3BSD SPMS README](https://www.tuhs.org/cgi-bin/utree.pl?file=4.3BSD%2Fusr%2Fcontrib%2Fspms%2FREADME) | BSD Software Project Management System |
| 1980s | [CMU SEI SCM](https://www.sei.cmu.edu/documents/5920/Support_Materials_for_Software_Configuration_Management.pdf) | Support Materials for Software Configuration Management |
| 2013 | [vspms](https://github.com/rustt/vspms) | Shell scripts by Russ Tremain that inspired the `chpj`-style workflow |

> **Note:** The `chpj`-style commands were informal add-ons shared between developers, not part of the official SPMS distribution. Documentation from that era is hard to find online.

## License

MIT
