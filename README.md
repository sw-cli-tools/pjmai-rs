# PJMAI - Project Management CLI

A Rust CLI tool for managing and quickly switching between projects via shell aliases.

## Overview

PJMAI helps developers manage multiple projects by maintaining a registry of project directories and files. It integrates with your shell to enable quick project switching with short commands.

## Installation

### Quick Install (Recommended)

Install pjmai with a single command:

```bash
curl -fsSL https://raw.githubusercontent.com/wrightmikea/pjmai/main/install.sh | bash
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
curl -fsSL https://raw.githubusercontent.com/wrightmikea/pjmai/main/install.sh | bash -s -- --prefix /usr/local/bin

# Skip shell integration (configure manually later)
curl -fsSL https://raw.githubusercontent.com/wrightmikea/pjmai/main/install.sh | bash -s -- --no-shell

# Skip completions
curl -fsSL https://raw.githubusercontent.com/wrightmikea/pjmai/main/install.sh | bash -s -- --no-completions
```

### From Source (Manual)

```bash
# Clone the repository
git clone https://github.com/wrightmikea/pjmai.git
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
| `pjmai change -p <name>` | `chpj` | Switch to a project |
| `pjmai list` | `lspj` | List all projects |
| `pjmai push -p <name>` | `pspj` | Push current to stack, switch to project |
| `pjmai pop` | `popj` | Pop from stack, return to previous project |
| `pjmai remove -p <name>` | `rmpj` | Remove a project |
| `pjmai rename -f <old> -t <new>` | `mvpj` | Rename a project |
| `pjmai show` | `shpj` | Show current project (and stack) |
| `pjmai prompt` | `prpj` | Output current project name (for prompts) |
| `pjmai aliases` | `hlpj` | Show all available aliases |
| `pjmai complete projects [prefix]` | - | Fast project name completion for shells |
| `pjmai complete commands [prefix]` | - | Fast command name completion for shells |
| `pjmai completions <shell>` | - | Generate shell completions |
| `pjmai scan [dir]` | `scpj` | Scan for git repositories and add as projects |
| `pjmai context [-p project]` | `ctpj` | Show project context for AI agents |
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

The scan command:
- Finds directories containing `.git`
- Parses git remote origin to extract owner/organization
- Groups repositories by host/owner for display
- Suggests nicknames based on repo name (handles collisions)
- Honors `.gitignore` patterns when recursing
- Skips already-registered projects (by path)

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
chpj -p webapp

# Switch to devenv (sources the environment file)
chpj -p devenv

# Fuzzy matching: partial names work too
chpj -p web      # matches "webapp" if unique
chpj -p WEBAPP   # case-insensitive matching
```

The `change` command supports fuzzy matching:
- **Exact match**: `webapp` matches project named "webapp"
- **Case-insensitive**: `WEBAPP` matches "webapp"
- **Prefix match**: `web` matches "webapp" (if unique)
- **Substring match**: `app` matches "webapp" (if unique)
- **Ambiguous**: If multiple projects match, shows all matches

### Push/Pop Stack Navigation

Use push/pop for temporary project switches when you need to return:

```bash
# Working in webapp, need to check something in api
pspj api           # Push webapp to stack, switch to api

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

### Managing Your Project List

```bash
# View all projects (current project highlighted)
lspj

# Example output:
# >webapp   ~/code/my-webapp
#  backend  ~/code/backend-api
#  devenv   ~/envs/dev-environment.sh

# Remove a project you no longer need
rmpj -p oldproject
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

PJMAI uses exit codes to communicate with the shell wrapper script:

- **Exit 2**: The output is a directory path; the shell will `cd` to it
- **Exit 3**: The output is a file path; the shell will `source` it
- **Exit 4**: Error occurred; the shell displays the error message

This mechanism allows the CLI (which runs as a subprocess) to affect the parent shell's environment.

## Development

### Building

```bash
cargo build
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

## License

MIT
