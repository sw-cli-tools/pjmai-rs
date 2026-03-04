# PJMAI-RS Improvements Roadmap

## Table of Contents

- [Project Introduction](#project-introduction)
- [Background](#background)
- [Current State](#current-state)
- [Purpose and Goals](#purpose-and-goals)
- [Current Usage](#current-usage)
- [Limitations](#limitations)
- [Improvement Areas](#improvement-areas)
  - [1. Installation](#1-installation)
  - [2. Human Usability](#2-human-usability)
  - [3. AI Agent Usability](#3-ai-agent-usability)
  - [4. Shell Autocompletion](#4-shell-autocompletion)
  - [5. Project-Specific Environments](#5-project-specific-environments)
  - [6. AI Agent Sandboxing](#6-ai-agent-sandboxing)
  - [7. Project Grouping](#7-project-grouping)
  - [8. Configuration Management](#8-configuration-management)
  - [9. AI-Assisted Configuration](#9-ai-assisted-configuration)
- [Implementation Priority](#implementation-priority)
- [Technical Architecture Changes](#technical-architecture-changes)

---

## Project Introduction

**PJMAI-RS** (Project Manager AI - Rust) is a Rust CLI tool for managing and switching between software development projects. It maintains a registry of project "nicknames" mapped to filesystem paths, enabling rapid context switching via short shell aliases.

Key capabilities:
- **Project Registry**: Store project names with associated paths (directories or setup scripts)
- **Quick Switching**: Use `chpj <nickname>` to instantly `cd` to a project or source its setup script
- **Shell Integration**: Exit code signaling allows the CLI to affect the parent shell's environment
- **Fuzzy Matching**: Find projects by prefix, substring, or case-insensitive matching
- **Prompt Integration**: Display current project name in your shell prompt

---

## Background

PJMAI is a "clean room" reimplementation of an older project management tool. The original tool was created in an era before AI coding agents, when developer workflows were entirely human-driven.

The world has changed significantly:
- **AI coding agents** (Claude Code, Cursor, Cody, etc.) now work alongside developers
- **Sandboxing requirements** are increasingly important when running AI-generated code
- **Context-aware tooling** helps both humans and AI understand project structure
- **Environment isolation** (uv, conda, nvm, Docker) is standard practice

This creates an opportunity to evolve PJMAI from a simple project switcher into a comprehensive **project environment manager** that serves both human developers and AI agents.

---

## Current State

### What Works Well

| Feature | Status | Notes |
|---------|--------|-------|
| Add/remove projects | Stable | Full CRUD operations |
| Switch projects (cd) | Stable | Exit code 2 triggers shell cd |
| Source setup scripts | Stable | Exit code 3 triggers shell source |
| List projects | Stable | Shows current project highlighted |
| Fuzzy matching | Stable | Prefix, substring, case-insensitive |
| Shell aliases | Stable | `adpj`, `chpj`, `lspj`, `rmpj`, `shpj`, `prpj`, `hlpj` |
| Prompt integration | Stable | `prpj` outputs current project name |
| Tab completion | Partial | Basic completions via `source-pjm.sh` |
| Shell completions | Generated | `pjmai completions <shell>` |
| Logging | Stable | `-l` flag with env_logger |

### Known Issues

1. **Serde dependency outdated**: Run `cargo update -p serde_derive` to fix warnings
2. **Debug flag unimplemented**: `-d` flag calls `unimplemented!()`
3. **Stale current_project**: Manual `cd` doesn't update tracking
4. **Limited path expansion**: Only `~` is expanded, not `$VAR` or other shell features

---

## Purpose and Goals

### Original Purpose
Enable developers to quickly switch between projects without remembering or typing full paths.

### Evolved Goals

1. **Human Usability**: Remain simple and fast for daily developer use
2. **AI Agent Support**: Provide machine-readable output and context for AI coding agents
3. **Environment Management**: Configure project-specific environments, paths, and restrictions
4. **Sandboxing**: Integrate with tools like `nono` (anti-sudo), `uv venv`, and containers
5. **Discoverability**: Help users (human and AI) understand available projects and their contexts

---

## Current Usage

### Basic Workflow

```bash
# Setup (one time)
source /path/to/source-pjm.sh

# Add a project
adpj myproject -f ~/code/myproject

# Switch to project (cd happens automatically)
chpj myproject

# List all projects
lspj

# Show current project
shpj

# Add to prompt
export PS1='[$(prpj)] \w $ '
```

### File-Based Projects (Environment Setup)

```bash
# Create a setup script
cat > ~/envs/webapp-env.sh << 'EOF'
#!/bin/bash
cd ~/code/webapp
source .venv/bin/activate
export DATABASE_URL="postgres://localhost/webapp_dev"
export NODE_ENV=development
EOF

# Register it
adpj webapp -f ~/envs/webapp-env.sh

# Switch (sources the script)
chpj webapp
```

---

## Limitations

### For Humans

| Limitation | Impact | Difficulty to Fix |
|------------|--------|-------------------|
| No tab completion for nicknames | Must remember exact names | Easy |
| No project groups/tags | Hard to organize many projects | Medium |
| No backup/restore | Config loss on machine change | Easy |
| No import from other tools | Manual migration required | Medium |
| No project templates | Repetitive setup for similar projects | Medium |

### For AI Agents

| Limitation | Impact | Difficulty to Fix |
|------------|--------|-------------------|
| No machine-readable output | Must parse human output | Easy |
| No project metadata | AI lacks context about projects | Medium |
| No environment restrictions | Can't limit AI access | Hard |
| No sandbox integration | No safe execution environment | Hard |
| No semantic help | Error messages not AI-optimized | Medium |

---

## Improvement Areas

### 1. Installation

#### Current Process (Manual)
```bash
cargo build --release
mkdir -p ~/.local/bin
cp target/release/pjmai ~/.local/bin/
echo 'source /path/to/source-pjm.sh' >> ~/.zshrc
```

#### Proposed Improvements

**A. Install Script**
```bash
# One-line install
curl -fsSL https://raw.githubusercontent.com/wrightmikea/pjmai/master/install.sh | bash
```

The install script would:
1. Detect OS and architecture
2. Download or build the binary
3. Install to `~/.local/bin/` or `/usr/local/bin/`
4. Download and install `source-pjm.sh`
5. Detect shell (bash/zsh/fish) and add to appropriate rc file
6. Generate and install shell completions
7. Run `pjmai --help` to verify

**B. Homebrew Formula**
```bash
brew tap wrightmikea/pjmai
brew install pjmai
```

**C. Cargo Install**
```bash
cargo install pjmai
# + manual shell setup, or:
pjmai setup --shell  # New command to configure shell
```

**D. Nix Flake**
```nix
{
  inputs.pjmai.url = "github:wrightmikea/pjmai";
  # ...
}
```

**E. Self-Setup Command**
```bash
# New command to handle shell integration
pjmai setup --shell zsh    # Adds to ~/.zshrc
pjmai setup --completions  # Installs shell completions
pjmai setup --all          # Full setup
```

---

### 2. Human Usability

#### A. Interactive Mode

```bash
# Add project interactively
$ adpj
Project nickname: mywebapp
Path [~/]: ~/code/mywebapp
Description (optional): Main web application
Tags (comma-separated): web, python, work
Added project 'mywebapp' -> ~/code/mywebapp

# Switch interactively with fuzzy finder (requires fzf)
$ chpj
> web
  mywebapp     ~/code/mywebapp     [web, python, work]
  webapi       ~/code/webapi       [web, rust, work]

# Or integrate with fzf directly
chpj $(lspj --names-only | fzf)
```

#### B. Rich Output

```bash
$ lspj --detailed
┌────────────┬─────────────────────────┬──────────────────┬───────────┐
│ Project    │ Path                    │ Tags             │ Last Used │
├────────────┼─────────────────────────┼──────────────────┼───────────┤
│ ▶ pjmai     │ ~/github/wrightmikea/pjmai │ rust, cli, tools │ 2 min ago │
│   webapp   │ ~/code/webapp           │ web, python      │ 1 day ago │
│   dotfiles │ ~/dotfiles              │ config           │ 3 days ago│
└────────────┴─────────────────────────┴──────────────────┴───────────┘
```

#### C. Recently Used Ordering

```bash
# Switch to most recently used project
chpj -       # Like `cd -`
chpj --last  # Same as above

# List by recency
lspj --recent
```

#### D. Project Notes

```bash
# Add notes to a project
pjmai note mywebapp "Deploy branch is production"
pjmai note mywebapp "Run migrations before testing"

# Show notes when switching
$ chpj mywebapp
📁 Switching to mywebapp
📝 Notes:
   - Deploy branch is production
   - Run migrations before testing
```

---

### 3. AI Agent Usability

This is a critical improvement area. AI coding agents need:

1. **Machine-readable output** for parsing
2. **Rich context** about projects
3. **Clear error messages** with suggested fixes
4. **Discovery mechanisms** to understand the project landscape

#### A. JSON Output Mode

```bash
# Machine-readable output for all commands
$ pjmai --json list
{
  "projects": [
    {
      "name": "pjmai",
      "path": "/Users/mike/github/wrightmikea/pjmai",
      "type": "directory",
      "is_current": true,
      "tags": ["rust", "cli"],
      "description": "Project manager CLI",
      "last_used": "2024-01-15T10:30:00Z"
    }
  ],
  "current_project": "pjmai",
  "total": 5
}

$ pjmai --json show
{
  "name": "pjmai",
  "path": "/Users/mike/github/wrightmikea/pjmai",
  "type": "directory",
  "is_current": true
}

# Errors also in JSON
$ pjmai --json change nonexistent
{
  "error": "project_not_found",
  "message": "No project found matching 'nonexistent'",
  "suggestions": ["project1", "project2"],
  "hint": "Use 'pjmai list' to see all projects"
}
```

#### B. AI-Friendly Help

```bash
$ pjmai help --ai
# PJMAI CLI - AI Agent Reference

## Purpose
Manage and switch between development projects via nicknames.

## Commands (with expected outputs)

### List projects
Command: pjmai list
Output format: One project per line, current project marked with ">"
Example:
  > project1  ~/path/to/project1
    project2  ~/path/to/project2

### Switch to project
Command: pjmai change <name>
Exit codes:
  - 2: Success, output is directory path (shell will cd)
  - 3: Success, output is file path (shell will source)
  - 4: Error, output is error message

### Add project
Command: pjmai add -p <name> -f <path>
Requirements:
  - Path must exist
  - Name must be unique

## Error Handling
All errors include:
  - Error code for programmatic handling
  - Human-readable message
  - Suggested fix when applicable

## Exit Codes
0: Success (informational output)
2: Success (trigger cd)
3: Success (trigger source)
4: Error
```

#### C. Project Metadata for AI Context

```toml
# ~/.pjmai/config.toml (extended format)
version = "pjmai-0.2.0"
current_project = "pjmai"

[[project]]
name = "pjmai"
[project.action]
file_or_dir = "~/github/wrightmikea/pjmai"
[project.metadata]
description = "Project manager CLI in Rust"
language = "rust"
build_command = "cargo build"
test_command = "cargo test"
lint_command = "cargo clippy"
tags = ["cli", "rust", "productivity"]
readme = "README.md"
# AI-specific context
ai_context = """
This is a Rust CLI application using clap for argument parsing.
Shell integration is critical - test with source-pjm.sh.
All public items require doc comments (deny(missing_docs)).
"""
```

#### D. Context Command for AI Agents

```bash
# Output project context in a format suitable for AI system prompts
$ pjmai context pjmai
Project: pjmai
Path: /Users/mike/github/wrightmikea/pjmai
Type: Rust CLI application

Build: cargo build
Test: cargo test
Lint: cargo clippy --all-targets --all-features

Key Files:
- README.md: Project documentation
- CLAUDE.md: AI assistant instructions
- src/main.rs: Entry point
- Cargo.toml: Dependencies and configuration

Notes:
- All warnings treated as errors
- All public items need doc comments
- Shell integration via exit codes

# Or in a format directly usable as context
$ pjmai context pjmai --for-agent
# Outputs content suitable for inclusion in AI agent context
```

#### E. Semantic Error Messages

Current:
```
Error: Project with name 'web' not found
```

Improved:
```
Error: No project found matching 'web'

Did you mean one of these?
  webapp     ~/code/webapp
  webapi     ~/code/webapi

Tip: Use 'pjmai list' to see all projects, or 'pjmai list --filter web' to search.
```

For AI (with `--json`):
```json
{
  "error": {
    "code": "PROJECT_NOT_FOUND",
    "message": "No project found matching 'web'",
    "input": "web",
    "similar_projects": ["webapp", "webapi"],
    "commands_to_try": [
      "pjmai list",
      "pjmai list --filter web",
      "pjmai add -p web -f <path>"
    ]
  }
}
```

---

### 4. Shell Autocompletion

#### Current State

Tab completion exists via `source-pjm.sh`:
```bash
_pjm_projects() {
    local projects
    projects=$(lspj 2>/dev/null | awk '{print $1}' | tr -d '>')
    COMPREPLY=($(compgen -W "$projects" -- "${COMP_WORDS[COMP_CWORD]}"))
}
complete -F _pjm_projects chpj rmpj shpj
```

#### Proposed Improvements

**A. Fast Native Completions**

```bash
# Generate optimized completions
$ pjmai completions bash --with-projects > ~/.local/share/bash-completion/completions/pjmai

# The completion script calls pjmai directly for dynamic project list
_pjmai_complete() {
    local cur="${COMP_WORDS[COMP_CWORD]}"
    local cmd="${COMP_WORDS[1]}"

    case "$cmd" in
        change|remove|show|context)
            # Get project names directly from pjmai (fast, cached)
            COMPREPLY=($(pjmai complete projects "$cur" 2>/dev/null))
            ;;
        *)
            COMPREPLY=($(pjmai complete commands "$cur" 2>/dev/null))
            ;;
    esac
}
```

**B. Completion Subcommand**

```bash
# Fast, purpose-built completion helper
$ pjmai complete projects web
webapp
webapi
webserver

$ pjmai complete commands ch
change

$ pjmai complete tags ru
rust
ruby
```

**C. Rich Zsh Completions**

```zsh
# Show descriptions with completions
$ chpj <TAB>
webapp    -- Main web application (Python, Django)
webapi    -- REST API service (Rust)
pjmai      -- This project (Rust CLI)
```

**D. Fish Completions**

```fish
# fish-specific completions with descriptions
complete -c pjmai -n "__fish_use_subcommand" -a change -d "Switch to a project"
complete -c pjmai -n "__fish_seen_subcommand_from change" -a "(pjmai complete projects)"
```

---

### 5. Project-Specific Environments

This enables PJMAI to manage complete development environments, not just directories.

#### A. Environment Files

```bash
# Define environment in project config
[[project]]
name = "webapp"
[project.action]
file_or_dir = "~/code/webapp"
[project.environment]
# Variables to set when switching to this project
vars = { NODE_ENV = "development", DATABASE_URL = "postgres://localhost/webapp" }
# Path modifications
path_prepend = ["./node_modules/.bin", "./.venv/bin"]
path_remove = []  # Patterns to remove from PATH
# Source these files
source_files = [".envrc", ".env.local"]
# Run these commands on entry
on_enter = ["nvm use", "source .venv/bin/activate"]
# Run on exit (when switching away)
on_exit = ["deactivate"]
```

#### B. Virtual Environment Integration

```bash
# Python with uv
$ pjmai add webapp -f ~/code/webapp --venv uv
# Creates/activates venv automatically on chpj

# Node with nvm
$ pjmai add frontend -f ~/code/frontend --node 18
# Runs 'nvm use 18' on entry

# Multiple runtimes
$ pjmai add fullstack -f ~/code/fullstack --venv uv --node 20 --ruby 3.2
```

#### C. Direnv Integration

```bash
# If .envrc exists in project, automatically integrate
$ chpj webapp
direnv: loading ~/code/webapp/.envrc
direnv: export +DATABASE_URL +NODE_ENV
📁 Switched to webapp
```

---

### 6. AI Agent Sandboxing

This is an advanced feature to provide secure, isolated environments for AI coding agents.

#### A. Restricted PATH for AI Agents

```toml
[[project]]
name = "ai-sandbox"
[project.ai_agent]
enabled = true
# Only these commands available to AI
allowed_commands = [
    "cargo", "rustc", "rustfmt",
    "git", "gh",
    "cat", "ls", "find", "grep",
    "pjmai"
]
# Or exclude dangerous commands
blocked_commands = [
    "rm", "sudo", "curl", "wget",
    "ssh", "scp", "rsync"
]
# Custom PATH with only safe binaries
restricted_path = "~/.pjmai/sandboxed-bins"
```

```bash
# Generate restricted PATH environment
$ pjmai sandbox-setup ai-sandbox
Creating sandboxed environment for 'ai-sandbox'...
Symlinking allowed commands to ~/.pjmai/sandboxed-bins/
  ✓ cargo -> /Users/mike/.cargo/bin/cargo
  ✓ git -> /usr/bin/git
  ...
Created ~/.pjmai/envs/ai-sandbox.sh

# When AI agent activates project
$ chpj ai-sandbox --agent
🔒 AI Agent mode: restricted PATH active
Allowed: cargo, git, ls, find, grep, pjmai
Blocked: rm, sudo, curl, wget, ssh
```

#### B. Nono (Anti-Sudo) Integration

[nono-rs](https://docs.rs/crate/nono-rs/latest) is a Rust crate that intercepts and blocks sudo/privileged commands. See also the [nono-rs GitHub repository](https://github.com/your-repo/nono-rs) for usage examples.

```toml
[[project]]
name = "untrusted-code"
[project.sandbox]
use_nono = true  # Intercept sudo attempts
nono_mode = "deny"  # deny, log, or prompt
```

```bash
$ chpj untrusted-code --agent
🔒 Nono active: sudo commands will be blocked
```

#### C. Container Integration

```toml
[[project]]
name = "isolated-dev"
[project.container]
type = "docker"  # or "podman", "lxc", "lima"
image = "rust:1.75-slim"
mounts = [
    { host = "~/code/myproject", container = "/workspace", mode = "rw" },
    { host = "~/.cargo", container = "/root/.cargo", mode = "ro" }
]
ports = ["3000:3000", "5432:5432"]
# Enter container on project switch
enter_on_switch = true
```

```bash
$ chpj isolated-dev
🐳 Starting container for isolated-dev...
Container running: rust:1.75-slim
Mounted: ~/code/myproject -> /workspace
root@container:/workspace#
```

#### D. VM Integration (Advanced)

```toml
[[project]]
name = "secure-audit"
[project.vm]
type = "lima"  # or "orbstack", "multipass"
template = "ubuntu-22.04"
cpus = 2
memory = "4GiB"
```

#### E. Agent Context Injection

```toml
[[project]]
name = "webapp"
[project.ai_agent]
# Context automatically provided to AI agents
system_prompt = """
This is a Python Django web application.
- Always run tests with: python manage.py test
- Format code with: black .
- The database is PostgreSQL
- Never modify migration files directly
"""
# Files the agent should read first
context_files = ["README.md", "CONTRIBUTING.md", ".claude/context.md"]
# Files/patterns the agent should NOT modify
protected_files = ["*.lock", "migrations/*", ".env"]
```

---

### 7. Project Grouping

Organize projects logically for easier management.

#### A. Tags

```bash
# Add tags when creating
$ adpj myproject -f ~/code/myproject --tags work,python,web

# Add tags to existing project
$ pjmai tag myproject add frontend

# List by tag
$ lspj --tag work
  webapp     ~/code/webapp           [work, python]
  api        ~/code/api              [work, rust]

$ lspj --tags python,web
  webapp     ~/code/webapp           [work, python, web]
```

#### B. Groups (GitHub Organization Style)

```bash
# Create a group
$ pjmai group create work "Work projects"
$ pjmai group create personal "Personal projects"
$ pjmai group create oss "Open source contributions"

# Add projects to groups
$ pjmai group add work webapp api admin-panel
$ pjmai group add oss pjmai rtt1

# List by group
$ lspj --group work
[work]
  webapp     ~/code/work/webapp
  api        ~/code/work/api

$ lspj --group oss
[oss]
  pjmai       ~/github/wrightmikea/pjmai
  rtt1       ~/github/wrightmikea/rtt1

# Switch to a random project in group (for variety)
$ chpj --random --group oss
```

#### C. Auto-Grouping

```bash
# Auto-detect groups from path patterns
$ pjmai auto-group
Detected groups:
  github/wrightmikea (3 projects)
  github/work-org (5 projects)
  code/personal (2 projects)

Apply these groups? [Y/n]
```

#### D. Group-Level Settings

```toml
[groups.work]
description = "Work projects"
# Default environment for all projects in this group
default_env = { JIRA_URL = "https://work.atlassian.net" }
# Shared pre/post commands
on_enter = ["echo 'Work mode: remember to log time!'"]
```

---

### 8. Configuration Management

#### A. Export/Import

```bash
# Export full configuration
$ pjmai config export > pjm-backup.toml
$ pjmai config export --format json > pjm-backup.json

# Import configuration
$ pjmai config import pjm-backup.toml
Importing 15 projects...
  ✓ 12 new projects added
  ⚠ 3 projects already exist (skipped)

# Merge configurations
$ pjmai config import colleague-config.toml --merge
```

#### B. Sync Across Machines

```bash
# Store config in dotfiles repo
$ pjmai config link ~/dotfiles/pjm/config.toml
Configuration linked to ~/dotfiles/pjm/config.toml
Changes will be reflected automatically.

# Or use Git directly
$ cd ~/.pjm && git init
$ git remote add origin git@github.com:user/pjm-config.git
$ pjmai config sync  # git pull && git add . && git commit && git push
```

#### C. Templates

```bash
# Create project from template
$ pjmai template create rust-cli
Template 'rust-cli' created with settings:
  - Tags: rust, cli
  - Build: cargo build
  - Test: cargo test
  - Environment: RUST_BACKTRACE=1

# Use template
$ pjmai add newcli -f ~/code/newcli --template rust-cli
```

#### D. Machine-Specific Overrides

```toml
# ~/.pjmai/config.toml
[[project]]
name = "webapp"
[project.action]
file_or_dir = "~/code/webapp"

# ~/.pjmai/config.local.toml (not synced, machine-specific)
[[project.override]]
name = "webapp"
[project.environment]
# Different database on this machine
vars = { DATABASE_URL = "postgres://localhost:5433/webapp_dev" }
```

---

### 9. AI-Assisted Configuration

Use AI to help configure and manage projects.

#### A. Smart Project Discovery

```bash
$ pjmai discover
Scanning for projects...
Found 12 potential projects:

  ~/code/webapp           (Python, Django, has .env)
  ~/code/api              (Rust, Cargo.toml found)
  ~/code/frontend         (Node, package.json found)
  ~/github/wrightmikea/*  (4 Rust projects)

Add all discovered projects? [Y/n/select]
```

#### B. AI-Assisted Setup

```bash
$ pjmai setup-with-ai webapp
🤖 Analyzing ~/code/webapp...

Detected:
  - Python 3.11 project (pyproject.toml)
  - Django web framework
  - PostgreSQL database (from settings.py)
  - Docker Compose available

Suggested configuration:
  Name: webapp
  Tags: python, django, web, work
  Build: pip install -e .
  Test: pytest
  Environment:
    DJANGO_SETTINGS_MODULE=webapp.settings
    DATABASE_URL=postgres://localhost/webapp
  On Enter:
    source .venv/bin/activate

Accept this configuration? [Y/n/edit]
```

#### C. Integration with Local AI (Ollama)

```bash
# Configure local AI for privacy
$ pjmai config set ai.provider ollama
$ pjmai config set ai.model codellama

# Use AI for project analysis
$ pjmai analyze webapp
🤖 Analyzing webapp with Ollama (codellama)...

Project Summary:
  A Django web application for managing customer orders.
  Main entry point: manage.py
  Key dependencies: Django 4.2, psycopg2, celery

Suggested improvements:
  1. Add .pjmairc for environment setup
  2. Create docker-compose for local development
  3. Add pre-commit hooks for code quality
```

#### D. Natural Language Commands

```bash
$ pjmai ask "which project uses Django?"
Projects using Django:
  - webapp (~/code/webapp)
  - admin-panel (~/code/admin)

$ pjmai ask "switch to my rust CLI project"
Found: pjmai (~/github/wrightmikea/pjmai)
Switch to pjmai? [Y/n]
```

---

## Implementation Priority

### Phase 1: Foundation (v0.2.0) ✅ COMPLETE
*Focus: Core improvements for immediate usability*

| Feature | Status | Notes |
|---------|--------|-------|
| JSON output mode (`--json` flag) | ✅ Done | All commands support `--json` |
| Improved error messages with suggestions | ✅ Done | Includes `similar_projects` in errors |
| Install script (`install.sh`) | ✅ Done | Supports `--prompt`, `--scan-base` |
| `pjmai setup` command | ✅ Done | Shell integration + completions + prompt |
| Enhanced tab completion | ✅ Done | `pjmai complete projects [prefix]` |
| Implement debug flag | ✅ Done | `-d` shows config paths |

**Bonus features implemented (beyond original roadmap):**

| Feature | Alias | Notes |
|---------|-------|-------|
| Scan for git repos | `scpj` | Auto-discover projects with `--dry-run`, `--add-all` |
| Rename projects | `mvpj` | Change project nicknames |
| Push/pop stack navigation | `pspj`, `popj` | Temporary project switches with return |
| Prompt with stack depth | - | Shows `[project:N]` where N is stack size |
| `.pjmai.sh` env files | `srcpj` | Secure hash-based approval workflow |
| Uninstall script | - | `./uninstall.sh` with `--keep-config` |
| VHS demo tapes | - | 6 demos in `demo/` directory |

### Phase 2: Metadata (v0.3.0) ✅ COMPLETE
*Focus: Richer project information*

| Feature | Status | Notes |
|---------|--------|-------|
| Project metadata (description, tags, language) | ✅ Done | `adpj --description --tags --language --group` |
| Project groups | ✅ Done | Filter with `lspj --group` |
| `pjmai context` command for AI agents | ✅ Done | `ctpj` alias, `--for-agent` flag |
| Recently-used tracking | ✅ Done | `last_used` timestamp, `lspj --recent` |
| Configuration export/import | ✅ Done | `pjmai config export/import` |
| Project notes | ✅ Done | `pjmai note add/list/remove/clear` |

**New commands added:**
- `pjmai context [-p project] [--for-agent]` - Show project context (alias: `ctpj`)
- `pjmai tag -p <project> add/list/remove/clear` - Manage project tags
- `pjmai note -p <project> add/list/remove/clear` - Manage project notes
- `pjmai meta -p <project> --description/--language/--group` - Update metadata
- `pjmai list --tag/--group/--recent` - Filter and sort project list
- `pjmai complete tags/groups [prefix]` - Tab completion for tags/groups

### Phase 3: Environments (v0.4.0) ✅ COMPLETE
*Focus: Environment management*

| Feature | Status | Notes |
|---------|--------|-------|
| Environment variables per project | ✅ Done | `pjmai env set/unset` |
| on_enter hooks | ✅ Done | `pjmai env on-enter` |
| Shell integration (exit code 5) | ✅ Done | Executes env setup script |
| evpj alias | ✅ Done | Shell alias for env commands |

**New commands added:**
- `pjmai env -p <project> set KEY VALUE` - Set environment variable
- `pjmai env -p <project> unset KEY` - Remove environment variable
- `pjmai env -p <project> on-enter CMD` - Add entry hook command
- `pjmai env -p <project> show` - Display environment config
- `pjmai env -p <project> clear` - Clear all environment config

**Deferred to Phase 3.1:**

| Feature | Status | Notes |
|---------|--------|-------|
| on_exit hooks | ⏳ Pending | Commands to run when leaving project |
| path_prepend | ⏳ Pending | Modify PATH per project |
| uv/venv integration | ⏳ Pending | Auto-activate based on pyproject.toml |
| nvm integration | ⏳ Pending | Auto-run `nvm use` based on .nvmrc |
| Direnv compatibility | ⏳ Pending | Integration wrapper for .envrc |
| Templates | ⏳ Pending | Reusable environment configurations |

### Phase 4: AI & Sandboxing (v0.5.0) — NOT STARTED
*Focus: AI agent support and security*

| Feature | Status | Notes |
|---------|--------|-------|
| AI context injection | ⏳ Pending | |
| Restricted PATH mode | ⏳ Pending | |
| nono-rs integration | ⏳ Pending | See [nono-rs crate](https://docs.rs/crate/nono-rs/latest) |
| Protected files | ⏳ Pending | |
| `--agent` mode flag | ⏳ Pending | |
| AI-assisted discovery | ⏳ Pending | |
| **assist integration** | ⏳ Pending | See below |

#### Assist Integration (~/github/softwarewrighter/assist)

Integrate with the `assist` project's SQLite database for AI-generated project analysis:

**Commands:**
- `scpj --analyze` - Run assist on all discovered repos, populate AI metadata
- `shpj --analyze` - Run assist on current project, show AI summary
- `lspj --detailed` - Show assist-generated descriptions and status
- `pjmai context --ai` - Include assist observations in context output

**chpj integration:**
- On switch, show: "Last worked on: [date] - [assist summary]"
- Option: `chpj --verbose` to show full AI analysis
- Detect stale analysis: "Analysis is 30d old, run 'shpj --analyze' to refresh"

**Database sharing:**
- Read from assist's `~/.assist/assist.db` (observations table)
- Query by project path to find relevant entries
- Extract: recent work summaries, detected tech stack, common commands

### Phase 5: Containers (v0.6.0) — NOT STARTED
*Focus: Advanced isolation*

| Feature | Status | Notes |
|---------|--------|-------|
| Docker integration | ⏳ Pending | |
| LXC/Lima support | ⏳ Pending | |
| Per-project containers | ⏳ Pending | |
| Development environments as code | ⏳ Pending | |

---

## Technical Architecture Changes

### Config File Evolution

```toml
# Version 1 (current)
version = "pjmai-0.1.0"
current_project = "pjmai"

[[project]]
name = "pjmai"
[project.action]
file_or_dir = "~/github/wrightmikea/pjmai"

# Version 2 (proposed)
version = "pjmai-0.2.0"
current_project = "pjmai"

[settings]
json_output = false
ai_provider = "ollama"
ai_model = "codellama"

[[groups]]
name = "work"
description = "Work projects"

[[project]]
name = "pjmai"
group = "oss"
tags = ["rust", "cli"]
description = "Project manager CLI"
last_used = "2024-01-15T10:30:00Z"

[project.action]
file_or_dir = "~/github/wrightmikea/pjmai"

[project.metadata]
language = "rust"
build = "cargo build"
test = "cargo test"
readme = "README.md"

[project.environment]
vars = { RUST_BACKTRACE = "1" }
path_prepend = ["./target/debug"]

[project.ai_agent]
context_files = ["CLAUDE.md", "README.md"]
protected_files = ["Cargo.lock"]
```

### New Module Structure

```
src/
├── main.rs
├── lib.rs
├── args.rs          # CLI parsing
├── command/         # Command implementations
│   ├── mod.rs
│   ├── add.rs
│   ├── change.rs
│   ├── list.rs
│   ├── remove.rs
│   ├── show.rs
│   ├── context.rs   # NEW: AI context
│   ├── group.rs     # NEW: Group management
│   ├── config.rs    # NEW: Config management
│   └── setup.rs     # NEW: Shell setup
├── config/          # Configuration
│   ├── mod.rs
│   ├── types.rs
│   ├── migration.rs # Config version migration
│   └── local.rs     # Machine-specific overrides
├── environment/     # NEW: Environment management
│   ├── mod.rs
│   ├── vars.rs
│   ├── path.rs
│   └── hooks.rs
├── sandbox/         # NEW: Sandboxing
│   ├── mod.rs
│   ├── path.rs
│   ├── nono.rs
│   └── container.rs
├── ai/              # NEW: AI features
│   ├── mod.rs
│   ├── context.rs
│   ├── discover.rs
│   └── ollama.rs
├── output/          # NEW: Output formatting
│   ├── mod.rs
│   ├── human.rs
│   ├── json.rs
│   └── table.rs
├── projects.rs
├── io.rs
├── util.rs
└── error.rs
```

### Exit Code Expansion

```rust
pub enum ExitAction {
    Print = 0,           // Print output to console
    ChangeDirectory = 2, // Shell should cd
    SourceFile = 3,      // Shell should source
    Error = 4,           // Error occurred
    // New exit codes
    ExecCommand = 5,     // Shell should execute output as command
    SetEnv = 6,          // Shell should eval output as env setup
}
```

### Shell Integration Enhancement

```bash
# Enhanced source-pjm.sh
pjm_fn() {
    local PJM_OUT PJM_EXIT
    PJM_OUT=$(pjmai "$@")
    PJM_EXIT=$?

    case "$PJM_EXIT" in
        0) echo "$PJM_OUT";;
        2) cd "$PJM_OUT";;
        3) source "$PJM_OUT";;
        4) echo "Error: $PJM_OUT" >&2; return 1;;
        5) eval "$PJM_OUT";;           # Execute command
        6) eval "$PJM_OUT";;           # Set environment
        *) echo "$PJM_OUT";;
    esac
}

# Enhanced completions
_pjm_complete() {
    local cur="${COMP_WORDS[COMP_CWORD]}"
    local cmd="${COMP_WORDS[1]}"

    case "$cmd" in
        change|remove|show|context)
            COMPREPLY=($(pjmai complete projects "$cur" 2>/dev/null))
            ;;
        group)
            COMPREPLY=($(pjmai complete groups "$cur" 2>/dev/null))
            ;;
        tag)
            COMPREPLY=($(pjmai complete tags "$cur" 2>/dev/null))
            ;;
        *)
            COMPREPLY=($(pjmai complete commands "$cur" 2>/dev/null))
            ;;
    esac
}

complete -F _pjm_complete pjmai pjm_fn
complete -F _pjm_complete chpj rmpj shpj
```

---

## Conclusion

PJMAI has a solid foundation. The proposed improvements transform it from a simple project switcher into a comprehensive development environment manager that serves both human developers and AI coding agents.

Key themes:
1. **Machine-readable output** enables AI agent integration
2. **Environment management** reduces project setup friction
3. **Sandboxing** provides security for AI-generated code execution
4. **Grouping and metadata** scale to many projects
5. **Easy installation** lowers adoption barriers

The phased implementation approach allows incremental value delivery while building toward the complete vision.
