# Scanning for Projects

PJMAI can automatically discover git repositories and add them as projects.

## The Problem

"I have 200+ projects scattered across my filesystem."

Sound familiar? Developers who work on multiple projects, contribute to open source, or maintain work across different organizations often end up with dozens or hundreds of git repositories.

## The Solution

One command to find them all:

```bash
scpj ~/github --dry-run    # Preview what would be found
```

Output:
```
Scanning /Users/you/github...

Found 47 git repositories:
  github.com/your-company
    webapp       ~/github/work/webapp
    api-server   ~/github/work/api-server
  github.com/your-username
    dotfiles     ~/github/personal/dotfiles
    side-project ~/github/personal/side-project
  (no remote)
    local-experiments ~/github/experiments

Add all 47 project(s)? [Y/n]
```

Then add them all:
```bash
scpj ~/github              # Batch add with confirmation
scpj ~/github --add-all    # Skip confirmation
```

## The Payoff

Quick context switching with fuzzy matching:

```bash
chpj web<TAB>              # Tab completion
chpj webapp                # Instant context switch
lspj                       # See everything at a glance
```

## Post-Scan Cleanup

After scanning, you might want to rename some auto-generated nicknames:

```bash
# Rename a project
mvpj webapp2 work-webapp   # More descriptive name

# Remove unwanted projects
rmpj old-experiment        # Clean up
```

The scan generates unique nicknames automatically. When collisions occur, it appends numbers:
- `webapp` (first occurrence)
- `webapp2` (second occurrence with same name)

Use `mvpj` to give them more meaningful names.

## Command Reference

### Scan Command

```bash
pjmai scan [DIR] [OPTIONS]
```

**Arguments:**
- `DIR` - Starting directory (default: `~/`)

**Options:**
- `--depth N` - Maximum recursion depth (default: 3)
- `--ignore DIRS` - Comma-separated directory names to skip
- `--dry-run` - Show what would be found without adding
- `--add-all` - Add all found projects without confirmation

**Alias:** `scpj`

### Rename Command

```bash
pjmai rename -f <old-name> -t <new-name>
```

**Alias:** `mvpj <old-name> <new-name>`

### Remove Command

```bash
pjmai remove -p <name>
```

**Alias:** `rmpj <name>`

## What Gets Detected

The scan looks for directories containing a `.git` subdirectory and:

1. **Parses git remote origin** to extract:
   - Host (github.com, gitlab.com, etc.)
   - Owner/organization
   - Repository name

2. **Handles SSH aliases** like `github.com-work` for users with multiple SSH keys

3. **Falls back to directory name** if no remote is configured

## What Gets Skipped

**By default:**
- `node_modules`
- `target` (Rust)
- `vendor`
- `dist`
- `build`
- `__pycache__`
- `.venv`, `venv`
- Hidden directories (starting with `.`)
- Directories matching `.gitignore` patterns

**Already registered:**
- Projects with paths already in your config are skipped (prevents duplicates)

## Grouping by Organization

The scan groups repositories by host and owner for easy visualization:

```
github.com/your-company
  project-a    ~/work/project-a
  project-b    ~/work/project-b
github.com/your-username
  personal-1   ~/personal/project-1
  personal-2   ~/personal/project-2
```

This grouping is informational for now. Future versions will support project groups for switching context between organizations.

## Installation with Scan

You can scan during initial setup:

```bash
./install.sh --local . --scan-base ~/code
```

This installs pjmai and immediately scans the specified directory.

## Tips

1. **Start with `--dry-run`** to see what would be found before adding
2. **Use `--depth 2`** for faster scans of shallow hierarchies
3. **Add `--ignore`** for project-specific directories to skip
4. **Run incrementally** - scan different directories as needed
5. **Rename collisions** with `mvpj` for clearer nicknames
