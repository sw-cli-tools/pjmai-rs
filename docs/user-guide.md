# PJMAI-RS User Guide

A complete reference for all commands, with motivating use-cases and scenarios.

## Table of Contents

- [Concepts](#concepts)
- [Getting Started](#getting-started)
- [Project Navigation](#project-navigation)
  - [Switching Projects (chpj)](#switching-projects-chpj)
  - [Subdirectory Navigation](#subdirectory-navigation)
  - [Push/Pop Stack (pspj/popj)](#pushpop-stack-pspjpopj)
  - [Stack Management (stpj)](#stack-management-stpj)
  - [Navigation History (hypj)](#navigation-history-hypj)
- [Project Management](#project-management)
  - [Adding Projects (adpj)](#adding-projects-adpj)
  - [Listing Projects (lspj)](#listing-projects-lspj)
  - [Showing Current Project (shpj)](#showing-current-project-shpj)
  - [Removing Projects (rmpj)](#removing-projects-rmpj)
  - [Renaming Projects (mvpj)](#renaming-projects-mvpj)
  - [Scanning for Projects (scpj)](#scanning-for-projects-scpj)
- [Environment Configuration](#environment-configuration)
  - [Environment Variables (evpj)](#environment-variables-evpj)
  - [PATH Modifications](#path-modifications)
  - [Entry and Exit Hooks](#entry-and-exit-hooks)
  - [Auto-Detection](#auto-detection)
  - [Project Environment Files (.pjmai.sh)](#project-environment-files-pjmaish)
- [Metadata and Organization](#metadata-and-organization)
  - [Tags](#tags)
  - [Notes](#notes)
  - [Metadata Updates](#metadata-updates)
  - [Project Groups](#project-groups)
- [Shell Integration](#shell-integration)
  - [Shell Prompt (prpj)](#shell-prompt-prpj)
  - [Setup Command](#setup-command)
  - [Shell Completions](#shell-completions)
- [Scripting and Automation](#scripting-and-automation)
  - [JSON Output Mode](#json-output-mode)
  - [Non-Interactive Mode](#non-interactive-mode)
  - [AI Agent Context (ctpj)](#ai-agent-context-ctpj)
  - [Config Export/Import](#config-exportimport)
- [Alias Quick Reference](#alias-quick-reference)

---

## Concepts

PJMAI-RS is a project switcher for your terminal. It maintains a registry of named projects (stored in `~/.pjmai/config.toml`) and provides short shell aliases to navigate between them.

**Why not just use `cd`?** You can, but:
- `chpj api` is faster than `cd ~/github/work/company/api-server`
- Switching projects can automatically activate virtual environments, set env vars, and run setup commands
- You get a history of where you've been and a stack to return to
- Tab completion works on project names, not just filesystem paths

**Two navigation modes:**
- **`chpj` (change)** — Direct navigation. Goes to the project and clears the stack. Use this for "I'm done here, let me work on something else."
- **`pspj`/`popj` (push/pop)** — Stack-based navigation. Remembers where you were. Use this for "let me check something real quick and come back."

---

## Getting Started

```bash
# Install
curl -fsSL https://raw.githubusercontent.com/sw-cli-tools/pjmai-rs/main/install.sh | bash

# Add your first project
adpj myapp -f ~/code/myapp

# Switch to it
chpj myapp

# See what you have
lspj
```

---

## Project Navigation

### Switching Projects (chpj)

**Scenario:** You're working on the frontend and need to switch to the backend.

```bash
chpj backend
```

That's it. Your terminal is now in the backend project directory, with any configured environment variables set and hooks run.

**Fuzzy matching** means you don't need exact names:

```bash
chpj back       # prefix match — works if "backend" is the only match
chpj BACKEND    # case-insensitive
chpj end        # substring match
```

If multiple projects match, you'll see the candidates:

```bash
chpj web
# ambiguous project name 'web', matches: webapp, webapi, website
```

**Important:** By default, `chpj` clears the push/pop stack. If you had stacked projects via `pspj`, switching with `chpj` abandons that stack. You'll see a note:

```
note: Clearing stack (2 entries) — use pspj/popj or chpj --push for stack navigation
```

This is intentional — `chpj` means "I'm moving on," not "I'll be right back." Use `chpj --push` if you want stack behavior with chpj features (like subdirectory navigation).

### Subdirectory Navigation

**Scenario:** You want to jump straight into `src/lib/` within a project, not just the project root.

```bash
chpj myproject src/lib
```

Tab completion works at each level:

```bash
chpj myproject<TAB>           # complete project name
chpj myproject <TAB>          # list subdirs: src, tests, docs...
chpj myproject src/<TAB>      # list nested: lib, bin, main.rs...
chpj myproject src/lib<ENTER> # cd to ~/code/myproject/src/lib
```

Both space and slash syntax work, and you can mix them:

```bash
chpj myproject src lib        # space-separated
chpj myproject src/lib        # slash-separated
chpj myproject src/lib tests  # mixed
```

Error messages help when paths don't exist:

```bash
chpj myproject nonexistent
# subdirectory 'nonexistent' not found in project 'myproject'

chpj myproject README.md
# 'README.md' is a file, not a directory
```

### Push/Pop Stack (pspj/popj)

**Scenario:** You're deep in a debugging session on `webapp`, but need to quickly check the API docs in `api-server`, then come back.

```bash
# You're in webapp
pspj api-server     # saves webapp on the stack, switches to api-server
# — or equivalently —
chpj --push api-server  # same thing, with chpj features (subdirs, env setup)

# Check what you need...

popj                # returns to webapp
# Output (stderr): Returning to 'webapp' (stack now empty)
```

The `--push` flag on `chpj` gives you the best of both worlds — stack-based navigation with subdirectory support:

```bash
chpj --push api-server src/routes   # push current project, switch to api-server/src/routes
```

The stack supports multiple levels:

```bash
chpj project-a
pspj project-b     # stack: [project-a]
pspj project-c     # stack: [project-a, project-b]
popj                # back to project-b, stack: [project-a]
# Output: Returning to 'project-b' (1 remaining)
popj                # back to project-a, stack: []
# Output: Returning to 'project-a' (stack now empty)
popj                # no-op
# Output: warning: Stack is empty, staying in 'project-a'
```

**When does the stack get cleared?**
- Using `chpj` without `--push` (direct navigation abandons the stack)
- Using `stpj clear` (explicit clear)
- Running `source update.sh` (development reinstall)

### Stack Management (stpj)

**Scenario:** You've been pushing projects and lost track of what's on the stack.

```bash
# Show the stack
stpj show
# Stack (3):
#   top ->  project-c
#           project-b
#           project-a

# Something went wrong, clear it
stpj clear
# Cleared 3 entry(ies) from stack
```

JSON mode:

```bash
pjmai -j stack show
# {"stack": ["project-a", "project-b", "project-c"], "depth": 3}

pjmai -j stack clear
# {"stack": [], "depth": 0, "cleared": true}
```

### Navigation History (hypj)

**Scenario:** You were working on a project 20 minutes ago but can't remember which one. Or you frequently switch between the same few projects and want a quick way back.

```bash
# Show history (most recent last, like shell history)
hypj
#    1  webapp
#    2  api-server
#    3  config
#    4  webapp
# >  5  api-server

# Jump to an entry by number
hypj 1
# Switches to webapp (same as chpj webapp)

# Bad index is a no-op
hypj 99
# warning: History index 99 out of range (1-5)
```

History is:
- **Persistent** across terminal sessions (stored in config.toml)
- **Capped at 50 entries** (oldest entries are dropped)
- **Recorded on every navigation** — `chpj`, `pspj`, `popj`, and `hypj N` all add entries
- The `>` marker shows the current project

JSON mode:

```bash
pjmai -j history
# {"entries": [{"index": 1, "name": "webapp"}, ...], "total": 5}
```

---

## Project Management

### Adding Projects (adpj)

**Scenario:** You just cloned a new repo and want to register it.

```bash
# Basic: just a name and path
adpj myapp -f ~/code/myapp

# With metadata for organization
adpj myapp -f ~/code/myapp \
  --description "Customer portal" \
  --tags web,react,work \
  --language typescript \
  --group work
```

Projects can point to directories (most common) or files (sourced when switching):

```bash
# Directory project — chpj will cd here
adpj webapp -f ~/code/webapp

# File project — chpj will source this file
adpj devenv -f ~/envs/dev-setup.sh
```

### Listing Projects (lspj)

**Scenario:** You need to see all your registered projects, or find projects with a specific tag.

```bash
# List all (current project marked with >)
lspj
# > webapp   ~/code/webapp           [web, react]
#   backend  ~/code/backend          [rust, api]
#   dotfiles ~/dotfiles

# Filter by tag
lspj --tag rust

# Filter by group
lspj --group work

# Filter by language
lspj --lang rust

# Extended info (language, description, tags)
lspj --long

# Sort by recently used (tracks chpj/pspj/popj navigation)
lspj --recent

# Sort by filesystem modification time
lspj --modified
```

### Showing Current Project (shpj)

**Scenario:** You opened a terminal and forgot which project is active.

```bash
shpj
# > webapp   ~/code/webapp
#  Stack (2): api-server <- backend
```

The stack display tells you how deep you are in push/pop navigation.

### Removing Projects (rmpj)

Removes the project from the registry only — does not delete any files.

```bash
# Remove a single project
rmpj oldproject

# Remove ALL projects (prompts for confirmation)
rmpj --all
# This will remove all 47 project(s):
#   webapp (~/code/webapp)
#   backend (~/code/backend)
#   ...
# Remove all? [y/N]

# Skip the prompt
rmpj --all -y
```

If the removed project was on the push/pop stack, it's cleaned up there too. `--all` also clears the stack and history.

### Renaming Projects (mvpj)

**Scenario:** The auto-generated scan name is too generic.

```bash
mvpj webapp2 work-webapp
```

### Scanning for Projects (scpj)

**Scenario:** You have 50+ git repos scattered across `~/github/` and want to add them all at once.

```bash
# Preview what would be found
scpj ~/github --dry-run

# Add everything
scpj ~/github

# Deeper scan with exclusions
scpj ~/code --depth 5 --ignore node_modules,vendor

# Non-interactive: add all without prompting
scpj ~/code --add-all
```

**Fresh re-scan:** When you've reorganized repos (forks, org changes), clear everything and start over. User metadata (descriptions, tags, notes, last_used, env config) is preserved across reset:

```bash
# Clear all projects and re-scan in one step
scpj ~/github --reset

# Non-interactive fresh re-scan
scpj --reset -y ~/github

# Preview the fresh scan first
scpj ~/github --reset --dry-run
```

**Auto language detection:** Scan detects programming languages from project files (e.g., `Cargo.toml` → rust, `package.json` → javascript). Polyglot projects show combined languages like `rust+python`.

**Smart naming for collisions:** When the same repo name exists in multiple orgs, the scanner uses owner-prefixed names instead of numeric suffixes:

```
github.com/softwarewrighter
  foo         ~/github/softwarewrighter/foo     # first "foo" — no prefix needed
github.com/sw-cli-tools
  sw-cl-foo   ~/github/sw-cli-tools/foo         # owner-prefixed
github.com/sw-music-tools
  sw-mu-foo   ~/github/sw-music-tools/foo       # owner-prefixed
```

After scanning, use `mvpj` to rename any names you want to customize further.

### Editing Project Properties (edpj)

**Scenario:** You want to add a description, change the language, or pin a project after scanning.

```bash
# Set description and language
edpj webapp -D "Customer-facing portal" -L typescript

# Set group
edpj webapp -g work

# Pin a project (survives scan --reset)
edpj webapp --pin

# Unpin
edpj webapp --unpin
```

---

## Environment Configuration

### Environment Variables (evpj)

**Scenario:** Your project needs `DATABASE_URL` set when you're working on it.

```bash
# Set a variable
evpj webapp set DATABASE_URL "postgres://localhost/webapp"

# Remove it
evpj webapp unset DATABASE_URL

# View config
evpj webapp show
```

Variables are automatically exported when you `chpj` to the project.

### PATH Modifications

**Scenario:** You want project-local binaries available without full paths.

```bash
evpj webapp path-prepend "./.venv/bin"
evpj webapp path-prepend "./node_modules/.bin"

# Remove a path
evpj webapp path-remove "./node_modules/.bin"
```

Now `python` resolves to the venv and `eslint` resolves to the local install.

### Entry and Exit Hooks

**Scenario:** Activate a Python venv on entry, deactivate on exit.

```bash
evpj webapp on-enter "source .venv/bin/activate"
evpj webapp on-exit "deactivate"
```

When you switch:
1. On-exit hooks from the previous project run first
2. `cd` to the new project
3. PATH is modified
4. Variables are exported
5. On-exit hooks are stored for later
6. On-enter hooks run
7. `.pjmai.sh` is checked

### Auto-Detection

**Scenario:** You added a new project and don't want to manually configure the environment.

```bash
# Preview
evpj webapp auto-detect --dry-run

# Apply
evpj webapp auto-detect
```

Detects:
- Python venv (`.venv/`, `venv/`) — activate/deactivate + PATH
- Node.js (`.nvmrc`) — `nvm use`
- Node modules (`node_modules/.bin/`) — PATH
- Rust (`Cargo.toml`) — `./target/debug` in PATH
- direnv (`.envrc`) — suggests sourcing

### Project Environment Files (.pjmai.sh)

**Scenario:** You want per-project shell setup that lives in the repo.

Create `~/code/myproject/.pjmai.sh`:
```bash
source .venv/bin/activate
export SRCROOT="$PWD"
```

On first visit, PJMAI warns you (security):
```
Found .pjmai.sh - inspect: 'cat .pjmai.sh', approve: 'srcpj'
```

After you inspect and approve with `srcpj`, it auto-sources on future visits (until the file changes).

---

## Metadata and Organization

### Tags

```bash
pjmai tag -p webapp add frontend,react
pjmai tag -p webapp list
pjmai tag -p webapp remove react
pjmai tag -p webapp clear
```

Then filter: `lspj --tag frontend`

### Notes

**Scenario:** Leave yourself reminders about a project.

```bash
pjmai note -p webapp add "Deploy branch is 'production'"
pjmai note -p webapp add "Requires VPN for staging DB"
pjmai note -p webapp list
#  1. Deploy branch is 'production'
#  2. Requires VPN for staging DB
pjmai note -p webapp remove 1
```

### Metadata Updates

```bash
pjmai meta -p webapp \
  --description "Customer portal v2" \
  --language typescript \
  --group frontend
```

### Project Groups

Groups are inferred from directory structure — the parent directory becomes the group name:

```
~/github/
├── sw-cli-tools/        ← group "sw-cli-tools"
│   ├── pjmai-rs/
│   └── other-tool/
└── personal/            ← group "personal"
    └── dotfiles/
```

```bash
# List groups
lsgp
# GROUP              ALIAS   PROJECTS   PATH
# > sw-cli-tools     cli     2          ~/github/sw-cli-tools
#   personal                 1          ~/github/personal

# Show group details
shgp                         # current group
shgp personal                # specific group
shgp --all                   # with project list

# Group name for shell prompt
prgp                         # prints group name
prgp --alias                 # prints alias if set

# Set group aliases
pjmai group alias sw-cli-tools cli
pjmai group alias . myalias  # alias current group

# Filter projects by group
lspj --group cli
lspj --group .               # current group
```

---

## Shell Integration

### Shell Prompt (prpj)

**Scenario:** You want your prompt to show which project you're in.

```bash
# Automatic setup
pjmai setup --prompt
```

Or manually in `~/.zshrc`:
```bash
PROMPT='[$(prpj)] %~ $ '
```

Result:
```
[webapp] ~/code/webapp $           # normal
[api:2] ~/code/api $               # 2 items on push/pop stack
~/code $                           # no project set
```

### Setup Command

Auto-configure shell integration after a manual install:

```bash
pjmai setup              # auto-detect shell
pjmai setup zsh          # specify shell
pjmai setup --shell-only # only shell integration
pjmai setup --completions-only   # only completions
pjmai setup --prompt     # add prompt integration
```

### Shell Completions

Tab completion is built in for `chpj`, `rmpj`, and `pspj`. It uses the fast native `pjmai complete` command under the hood. Tab completion also works with `chpj --push` — the `--push` flag is transparent to completion, so `chpj --push r<TAB>` completes project names just like `pspj r<TAB>`.

For other tools, generate standard completions:

```bash
pjmai completions bash > ~/.local/share/bash-completion/completions/pjmai
pjmai completions zsh > ~/.zsh/completions/_pjmai
pjmai completions fish > ~/.config/fish/completions/pjmai.fish
```

---

## Scripting and Automation

### JSON Output Mode

**Scenario:** You're writing a script that needs to parse project data.

Every command supports `--json` / `-j`:

```bash
pjmai -j list
# {"projects": [...], "current_project": "webapp", "total": 5}

pjmai -j show
# {"name": "webapp", "path": "/Users/mike/code/webapp", "type": "directory"}

pjmai -j history
# {"entries": [{"index": 1, "name": "webapp"}, ...], "total": 3}

pjmai -j stack show
# {"stack": ["project-a"], "depth": 1}

# Errors are structured too
pjmai -j change -p nonexistent
# {"code": "PROJECT_NOT_FOUND", "message": "...", "hint": "..."}
```

### Non-Interactive Mode

**Scenario:** Automated setup in a CI script or demo recording.

```bash
pjmai -y list              # creates config without prompting
pjmai -y scan ~/code       # adds all found projects without confirmation
```

### AI Agent Context (ctpj)

**Scenario:** Give an AI assistant context about your project.

```bash
ctpj                       # current project
ctpj webapp                # specific project
ctpj webapp --for-agent    # optimized for system prompts
```

Shows: path, description, tags, language, key files (README, Cargo.toml, package.json, etc.), and notes.

### Config Export/Import

**Scenario:** Back up your project registry or share it across machines.

```bash
# Export
pjmai config export > backup.toml
pjmai config export --format json > backup.json

# Import
pjmai config import backup.toml
pjmai config import backup.toml --merge     # merge with existing
pjmai config import backup.toml --dry-run   # preview only
```

---

## Alias Quick Reference

| Alias | Command | Description |
|-------|---------|-------------|
| `adpj` | `pjmai add` | Add a new project |
| `chpj` | `pjmai change` | Switch to project (clears stack; `--push` to push instead) |
| `ctpj` | `pjmai context` | Show project context for AI |
| `edpj` | `pjmai edit` | Edit project properties (description, language, pin) |
| `evpj` | `pjmai env` | Manage environment config |
| `hlpj` | `pjmai aliases` | Show all aliases |
| `hypj` | `pjmai history` | Show or jump to navigation history |
| `lspj` | `pjmai list` | List all projects |
| `mvpj` | `pjmai rename` | Rename a project |
| `popj` | `pjmai pop` | Pop from stack, return to previous |
| `prpj` | `pjmai prompt` | Current project name for prompt |
| `pspj` | `pjmai push` | Push current, switch to project |
| `qypj` | `pjmai query` | Check if project exists (exit 0/1) |
| `rmpj` | `pjmai remove` | Remove a project |
| `scpj` | `pjmai scan` | Scan for git repositories |
| `shpj` | `pjmai show` | Show current project and stack |
| `stpj` | `pjmai stack` | Show or clear the project stack |
| `xppj` | `pjmai exports` | Export paths as named directories |
| `srcpj` | *(shell fn)* | Source and approve `.pjmai.sh` |
| `lsgp` | `pjmai group list` | List all groups |
| `shgp` | `pjmai group show` | Show group details |
| `prgp` | `pjmai group prompt` | Group name for prompt |

Run `hlpj` at any time to see this list in your terminal.
