# PJM1 - Project Management CLI

A Rust CLI tool for managing and quickly switching between projects via shell aliases.

![PJM1 Full Workflow Demo](demo/full-workflow.gif)

## Overview

PJM1 helps developers manage multiple projects by maintaining a registry of project directories and files. It integrates with your shell to enable quick project switching with short commands.

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/wrightmikea/pjm1.git
cd pjm1

# Build the binary
cargo build --release

# Copy to your PATH (create ~/.local/bin if needed)
mkdir -p ~/.local/bin
cp target/release/pjm1 ~/.local/bin/
```

### Shell Integration (Required)

Add the following to your shell's rc file:

**For Zsh (~/.zshrc) on macOS:**
```bash
# PJM1 project management
export PATH="$HOME/.local/bin:$PATH"
source /path/to/pjm1/source-pjm.sh
```

**For Bash (~/.bashrc) on Linux:**
```bash
# PJM1 project management
export PATH="$HOME/.local/bin:$PATH"
source /path/to/pjm1/source-pjm.sh
```

Then reload your shell: `source ~/.zshrc` (or `~/.bashrc`)

This sets up:
- Shell aliases (`adpj`, `chpj`, `lspj`, etc.)
- A wrapper function that allows PJM1 to change your working directory

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
| `pjm1 add -p <name> -f <path>` | `adpj` | Add a new project |
| `pjm1 change -p <name>` | `chpj` | Switch to a project |
| `pjm1 list` | `lspj` | List all projects |
| `pjm1 remove -p <name>` | `rmpj` | Remove a project |
| `pjm1 show` | `shpj` | Show current project |
| `pjm1 prompt` | `prpj` | Output current project name (for prompts) |
| `pjm1 aliases` | `hlpj` | Show all available aliases |
| `pjm1 completions <shell>` | - | Generate shell completions |

## Shell Completions

Generate and install shell completions for tab-completion support:

```bash
# Bash (add to ~/.bashrc)
pjm1 completions bash > ~/.local/share/bash-completion/completions/pjm1

# Zsh (add to ~/.zshrc or a file in your fpath)
pjm1 completions zsh > ~/.zsh/completions/_pjm1

# Fish
pjm1 completions fish > ~/.config/fish/completions/pjm1.fish

# PowerShell
pjm1 completions powershell >> $PROFILE
```

## Usage Scenarios

### Setting Up a New Project

When starting work on a new project, add it to PJM1:

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

PJM1 stores its configuration at `~/.pjm/config.toml`. The file is created automatically on first use.

### Custom Configuration Directory

Set the `PJM_CONFIG_DIR` environment variable to use a different configuration directory:

```bash
export PJM_CONFIG_DIR=~/my-custom-config
```

This is useful for testing or maintaining separate project registries.

## How It Works

PJM1 uses exit codes to communicate with the shell wrapper script:

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
RUST_LOG=info pjm1 -l list
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
