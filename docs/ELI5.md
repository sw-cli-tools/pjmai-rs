# PJMAI-RS: Explain Like I'm 5

A complete guide to managing your development projects with PJMAI-RS.

## Table of Contents

- [What is PJMAI?](#what-is-pjmai)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Core Commands](#core-commands)
  - [Adding Projects](#adding-projects)
  - [Switching Projects](#switching-projects)
  - [Listing Projects](#listing-projects)
  - [Showing Current Project](#showing-current-project)
  - [Removing Projects](#removing-projects)
  - [Renaming Projects](#renaming-projects)
- [Project Stack (Push/Pop)](#project-stack-pushpop)
- [Scanning for Projects](#scanning-for-projects)
- [Project Metadata](#project-metadata)
  - [Tags](#tags)
  - [Notes](#notes)
  - [Metadata Updates](#metadata-updates)
- [Environment Configuration](#environment-configuration)
  - [Environment Variables](#environment-variables)
  - [PATH Modifications](#path-modifications)
  - [Entry and Exit Hooks](#entry-and-exit-hooks)
  - [Auto-Detection](#auto-detection)
- [Project Context for AI](#project-context-for-ai)
- [Configuration Management](#configuration-management)
- [Shell Prompt Integration](#shell-prompt-integration)
- [JSON Output Mode](#json-output-mode)
- [Shell Aliases Reference](#shell-aliases-reference)
- [Project Groups](#project-groups)
  - [Listing Groups](#listing-groups)
  - [Showing Group Details](#showing-group-details)
  - [Group Prompt](#group-prompt)
  - [Group Aliases](#group-aliases)
  - [Filtering by Group](#filtering-by-group)

---

## What is PJMAI?

PJMAI (Project Manager AI) is a command-line tool that helps you:

1. **Switch between projects instantly** - No more typing long paths like `cd ~/code/work/frontend/webapp`. Just type `chpj webapp`.

2. **Set up project environments automatically** - When you switch to a project, PJMAI can activate Python virtual environments, set environment variables, run setup commands, and more.

3. **Keep track of your projects** - Add descriptions, tags, and notes to remember what each project is about.

4. **Work with AI coding assistants** - Provides machine-readable output (JSON) and context information for AI tools.

Think of it like bookmarks for your terminal, but smarter.

---

## Installation

### Quick Install (Recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/sw-cli-tools/pjmai-rs/main/install.sh | bash
```

This will:
- Build the binary from source
- Install it to `~/.local/bin/`
- Set up shell integration
- Install tab completions

### After Installation

Start a new terminal, or run:
```bash
source ~/.zshrc  # or ~/.bashrc
```

You should now have access to all the `*pj` aliases (like `chpj`, `lspj`, etc.)

---

## Quick Start

```bash
# 1. Add your first project
adpj myapp -f ~/code/myapp

# 2. Switch to it
chpj myapp

# 3. See all your projects
lspj

# 4. See which project you're in
shpj
```

That's it! You've just set up your first project.

---

## Core Commands

### Adding Projects

**Alias:** `adpj`
**Full command:** `pjmai add`

Add a project by giving it a short nickname and pointing to its directory:

```bash
# Basic usage
adpj myproject -f ~/code/myproject

# With metadata
adpj webapp -f ~/code/webapp \
  --description "Main customer portal" \
  --tags web,python,work \
  --language python \
  --group work
```

**Options:**
| Option | Short | Description |
|--------|-------|-------------|
| `--file-or-dir` | `-f` | Path to project directory (required) |
| `--description` | `-D` | Human-readable description |
| `--tags` | `-t` | Comma-separated tags |
| `--language` | `-L` | Primary programming language |
| `--group` | `-g` | Project group (e.g., "work", "personal") |

### Switching Projects

**Alias:** `chpj`
**Full command:** `pjmai change`

Switch to a project by name:

```bash
# Exact match
chpj webapp

# Fuzzy matching works too
chpj web      # Matches "webapp" if it's unique
chpj WEB      # Case-insensitive
```

When you switch:
1. Your terminal `cd`s to the project directory
2. Environment variables are set (if configured)
3. Entry hooks run (if configured)
4. `.pjmai.sh` is sourced (if approved)

### Listing Projects

**Alias:** `lspj`
**Full command:** `pjmai list`

See all your projects:

```bash
# Basic list
lspj

# Output:
# > webapp   ~/code/webapp           [web, python]
#   backend  ~/code/backend          [rust, api]
#   dotfiles ~/dotfiles              [config]

# Filter by tag
lspj --tag python

# Filter by group
lspj --group work

# Filter by language
lspj --lang rust

# Extended info (language, description, tags)
lspj --long

# Sort by recently used
lspj --recent

# Sort by filesystem modification time
lspj --modified
```

The `>` marker shows your current project.

### Showing Current Project

**Alias:** `shpj`
**Full command:** `pjmai show`

See details about your current project:

```bash
shpj

# Output:
# Current project: webapp
# Path: /Users/mike/code/webapp
# Type: directory
```

### Removing Projects

**Alias:** `rmpj`
**Full command:** `pjmai remove`

Remove a project from PJMAI (doesn't delete any files):

```bash
rmpj oldproject
```

### Renaming Projects

**Alias:** `mvpj`
**Full command:** `pjmai rename`

Change a project's nickname:

```bash
mvpj oldname newname
```

---

## Project Stack (Push/Pop)

Sometimes you need to quickly check another project and come back. Use push/pop:

**Push:** `pspj` (or `chpj --push`) - Save current project and switch
**Pop:** `popj` - Return to the saved project

```bash
# You're working on webapp
chpj webapp

# Quick detour to check something in backend
pspj backend    # Pushes webapp to stack, switches to backend
# — or equivalently —
chpj --push backend   # Same thing, with chpj features (subdirs, env setup)

# Done checking, return to webapp
popj            # Pops webapp from stack, switches back
```

You can push multiple projects:
```bash
chpj project1
pspj project2   # Stack: [project1]
pspj project3   # Stack: [project1, project2]
popj            # Back to project2
popj            # Back to project1
```

---

## Scanning for Projects

**Alias:** `scpj`
**Full command:** `pjmai scan`

Automatically find git repositories and add them as projects:

```bash
# Scan home directory (default depth: 3)
scpj ~

# Scan specific directory with deeper search
scpj ~/code --depth 5

# Preview without adding
scpj ~/code --dry-run

# Skip certain directories
scpj ~/code --ignore node_modules,vendor

# Add all found projects without prompting
scpj ~/code --add-all
```

The scanner:
- Finds all `.git` directories
- Parses remote URLs to suggest group names
- Handles SSH aliases (like `github.com-work`)

---

## Project Metadata

### Tags

Organize projects with tags:

```bash
# Add tags
pjmai tag -p webapp add frontend,react

# List tags
pjmai tag -p webapp list

# Remove tags
pjmai tag -p webapp remove react

# Clear all tags
pjmai tag -p webapp clear
```

Then filter by tag:
```bash
lspj --tag frontend
```

### Notes

Add reminders and notes to projects:

```bash
# Add a note
pjmai note -p webapp add "Remember to run migrations"
pjmai note -p webapp add "Deploy branch is 'production'"

# List notes
pjmai note -p webapp list

# Remove note by number
pjmai note -p webapp remove 1

# Clear all notes
pjmai note -p webapp clear
```

### Editing Project Properties

**Alias:** `edpj`
**Full command:** `pjmai edit`

Update project metadata after adding:

```bash
# Set description and language
edpj webapp -D "Customer-facing web portal" -L typescript

# Set group
edpj webapp -g work

# Pin a project (survives scan --reset)
edpj webapp --pin

# Unpin
edpj webapp --unpin
```

---

## Environment Configuration

This is where PJMAI really shines. Configure what happens when you enter/leave a project.

**Alias:** `evpj`
**Full command:** `pjmai env`

### Environment Variables

Set variables that apply when you're in a project:

```bash
# Set a variable
evpj webapp set DATABASE_URL "postgres://localhost/webapp"
evpj webapp set NODE_ENV "development"

# Remove a variable
evpj webapp unset DATABASE_URL

# View current config
evpj webapp show
```

### PATH Modifications

Add directories to your PATH when entering a project:

```bash
# Add to PATH
evpj webapp path-prepend "./.venv/bin"
evpj webapp path-prepend "./node_modules/.bin"

# Remove from PATH config
evpj webapp path-remove "./node_modules/.bin"
```

Now when you `chpj webapp`, you can run `python` from `.venv` or `eslint` from `node_modules` without full paths.

### Entry and Exit Hooks

Run commands when entering or leaving a project:

```bash
# Commands to run when ENTERING the project
evpj webapp on-enter "source .venv/bin/activate"
evpj webapp on-enter "nvm use"

# Commands to run when LEAVING the project
evpj webapp on-exit "deactivate"
```

**Example workflow:**
```bash
# Configure webapp
evpj webapp on-enter "source .venv/bin/activate"
evpj webapp on-exit "deactivate"

# Now when you switch...
chpj webapp     # Activates venv automatically
chpj other      # Runs "deactivate" first, then switches
```

### Auto-Detection

Let PJMAI figure out what your project needs:

```bash
# Preview what would be configured
evpj webapp auto-detect --dry-run

# Auto-detect and apply
evpj webapp auto-detect
```

Auto-detect recognizes:

| Feature | Detected By | What It Configures |
|---------|-------------|-------------------|
| Python venv | `.venv/` or `venv/` | activate/deactivate + PATH |
| Node.js | `.nvmrc` | `nvm use` command |
| Node modules | `node_modules/.bin/` | Adds to PATH |
| Rust | `Cargo.toml` | Adds `./target/debug` to PATH |
| direnv | `.envrc` | Suggests sourcing (with warning) |

**Example:**
```bash
$ evpj myproject auto-detect
Detected environment features for project myproject:
  python-venv (from .venv/)
    Path prepend: ./.venv/bin
    On enter: source .venv/bin/activate
    On exit: deactivate
  node-nvm (from .nvmrc)
    On enter: nvm use

Configuration applied.
```

### Viewing Full Environment Config

```bash
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
```

### Clearing Environment Config

```bash
evpj webapp clear
```

---

## Project Context for AI

**Alias:** `ctpj`
**Full command:** `pjmai context`

Get project context in a format useful for AI assistants:

```bash
# Show context for current project
ctpj

# Show context for specific project
ctpj webapp

# Format optimized for AI system prompts
ctpj webapp --for-agent
```

This shows:
- Project path and type
- Description and tags
- Key files (README.md, Cargo.toml, package.json, etc.)
- Notes

---

## Configuration Management

### Export

Backup your configuration:

```bash
# Export as TOML (default)
pjmai config export > backup.toml

# Export as JSON
pjmai config export --format json > backup.json
```

### Import

Restore or merge configurations:

```bash
# Replace current config
pjmai config import backup.toml

# Merge with existing (keeps existing projects, adds new ones)
pjmai config import backup.toml --merge

# Preview what would be imported
pjmai config import backup.toml --dry-run
```

---

## Shell Prompt Integration

Show your current project in your prompt:

**Alias:** `prpj`
**Full command:** `pjmai prompt`

### Zsh

Add to `~/.zshrc`:
```bash
PROMPT='[$(prpj)] %~ $ '
```

### Bash

Add to `~/.bashrc`:
```bash
PS1='[$(prpj)] \w $ '
```

Your prompt will show:
```
[webapp] ~/code/webapp $
```

Or if you have stacked projects:
```
[webapp:2] ~/code/webapp $    # 2 projects on stack
```

---

## JSON Output Mode

For scripts and AI tools, use JSON output:

```bash
# All commands support --json
pjmai --json list
pjmai --json show
pjmai --json change -p webapp

# With aliases (put -j before the command)
pjmai -j list
```

Example JSON output:
```json
{
  "projects": [
    {
      "name": "webapp",
      "path": "/Users/mike/code/webapp",
      "type": "directory",
      "is_current": true,
      "tags": ["web", "python"]
    }
  ],
  "current_project": "webapp",
  "total": 1
}
```

---

## Shell Aliases Reference

After installing PJMAI, you get these short aliases:

| Alias | Command | Description |
|-------|---------|-------------|
| `adpj` | `pjmai add` | Add a new project |
| `chpj` | `pjmai change` | Switch to a project (clears stack; `--push` to push instead) |
| `ctpj` | `pjmai context` | Show project context (for AI) |
| `edpj` | `pjmai edit` | Edit project properties (description, language, pin) |
| `evpj` | `pjmai env` | Manage environment config |
| `hlpj` | `pjmai aliases` | Show all aliases |
| `hypj` | `pjmai history` | Show or jump to navigation history |
| `lspj` | `pjmai list` | List all projects |
| `mvpj` | `pjmai rename` | Rename a project |
| `popj` | `pjmai pop` | Pop from project stack |
| `prpj` | `pjmai prompt` | Get current project for prompt |
| `pspj` | `pjmai push` | Push and switch project |
| `qypj` | `pjmai query` | Check if project exists (exit 0/1) |
| `rmpj` | `pjmai remove` | Remove a project (`--all` supported) |
| `scpj` | `pjmai scan` | Scan for git repos (`--reset` supported) |
| `shpj` | `pjmai show` | Show current project |
| `stpj` | `pjmai stack` | Show or clear project stack |
| `xppj` | `pjmai exports` | Export paths as named directories |
| `srcpj` | (shell function) | Source and approve `.pjmai.sh` |

**Group aliases:**

| Alias | Command | Description |
|-------|---------|-------------|
| `lsgp` | `pjmai group list` | List all groups |
| `shgp` | `pjmai group show` | Show current group details |
| `prgp` | `pjmai group prompt` | Get current group for prompt |

---

## Project Groups

Groups are **automatically inferred** from your project directory structure. The parent directory of each project becomes its group.

```
~/github/
├── sw-cli-tools/        ← group "sw-cli-tools"
│   ├── pjmai-rs/
│   └── other-tool/
├── softwarewrighter/    ← group "softwarewrighter"
│   └── webapp/
└── personal/            ← group "personal"
    └── dotfiles/
```

### Listing Groups

**Alias:** `lsgp`
**Full command:** `pjmai group list`

```bash
# List all groups
lsgp

# Output:
# GROUP              ALIAS     PROJECTS   PATH
# > sw-cli-tools     cli       2          ~/github/sw-cli-tools
#   softwarewrighter work      1          ~/github/softwarewrighter
#   personal                   1          ~/github/personal

# Show groups with their projects
lsgp --all
```

The `>` marker shows your current group (based on current project).

### Showing Group Details

**Alias:** `shgp`
**Full command:** `pjmai group show`

```bash
# Show current group
shgp

# Output:
# Group: sw-cli-tools
# Alias: cli
# Path: ~/github/sw-cli-tools
# Projects: 2

# Show a specific group
shgp personal

# Show group with project list
shgp --all
```

### Group Prompt

**Alias:** `prgp`
**Full command:** `pjmai group prompt`

Print group name for shell prompt integration:

```bash
# Get current group name
prgp

# Get alias instead (if set)
prgp --alias
```

**Shell prompt integration:**
```bash
PROMPT='[$(prpj):$(prgp)] %~ $ '
# Result: [pjmai-rs:cli] ~/github/sw-cli-tools/pjmai-rs $
```

### Group Aliases

Give friendly names to inferred groups:

```bash
# Set an alias
pjmai group alias sw-cli-tools cli
pjmai group alias softwarewrighter work
pjmai group alias personal home

# List all aliases
pjmai group alias --list

# Remove an alias
pjmai group alias sw-cli-tools --remove

# Alias current group (use "." for current)
pjmai group alias . myalias
```

### Filtering by Group

Filter project listings by group:

```bash
# Filter by group name
lspj --group cli
lspj --group sw-cli-tools  # same result if "cli" is alias

# Filter by current group
lspj --group .
```

---

## Tips and Tricks

### 1. Use Auto-Detect for New Projects

When you add a new project, run auto-detect to set up the environment:
```bash
adpj newproject -f ~/code/newproject
evpj newproject auto-detect
```

### 2. Use Tags for Organization

Group related projects with tags:
```bash
adpj proj1 -f ~/code/proj1 --tags work,python
adpj proj2 -f ~/code/proj2 --tags work,python
adpj hobby -f ~/code/hobby --tags personal,rust

# Find all work Python projects
lspj --tag work | grep python
```

### 3. Use Push/Pop for Quick Checks

Instead of remembering where you were:
```bash
pspj other-project    # Check something
popj                  # Return automatically
```

### 4. Scan Periodically

Keep your project list fresh:
```bash
scpj ~/code --dry-run  # See what's new
scpj ~/code            # Add new projects
```

### 5. Export Your Config

Back up regularly:
```bash
pjmai config export > ~/dotfiles/pjmai-config.toml
```

---

## Troubleshooting

### "Command not found: chpj"

Make sure shell integration is loaded:
```bash
source ~/.pjmai/source-pjm.sh
```

Or re-run setup:
```bash
pjmai setup
```

### "Project not found"

Check the exact name:
```bash
lspj
```

PJMAI does fuzzy matching, but it needs at least a partial match.

### Environment not activating

Check your configuration:
```bash
evpj myproject show
```

Make sure commands are correct (e.g., `source .venv/bin/activate` not just `activate`).

### Changes not taking effect

Start a new terminal, or:
```bash
source ~/.zshrc
```

---

## Getting Help

```bash
# General help
pjmai --help

# Help for a specific command
pjmai add --help
pjmai env --help

# Show all aliases
hlpj
```

---

## What's Next?

Now that you understand the basics:

1. **Add your existing projects** with `adpj` or `scpj`
2. **Set up environments** with `evpj auto-detect`
3. **Add tags and descriptions** to stay organized
4. **Export your config** to back it up

Happy project managing!
