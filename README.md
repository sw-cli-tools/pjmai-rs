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
| `pjmai remove -p <name>` | `rmpj` | Remove a project |
| `pjmai show` | `shpj` | Show current project |
| `pjmai prompt` | `prpj` | Output current project name (for prompts) |
| `pjmai aliases` | `hlpj` | Show all available aliases |
| `pjmai complete projects [prefix]` | - | Fast project name completion for shells |
| `pjmai complete commands [prefix]` | - | Fast command name completion for shells |
| `pjmai completions <shell>` | - | Generate shell completions |
| `pjmai setup [shell]` | - | Auto-configure shell integration |

### Global Flags

| Flag | Short | Description |
|------|-------|-------------|
| `--json` | `-j` | Output in JSON format for machine parsing |
| `--logging` | `-l` | Enable logging |
| `--debug` | `-d` | Enable debug mode |

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

### Shell Prompt Integration

Add the current project to your shell prompt:

```bash
# In your .bashrc or .zshrc
export PS1='[\$(prpj)] \w $ '
```

This displays the current project name in your prompt:

```
[webapp] ~/code/my-webapp $
```

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

## License

MIT
