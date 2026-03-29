# Usage Guide

## Installation

### Prerequisites

- Emacs 27.1 or later
- `pjmai` binary installed and in your `$PATH`

### Load from source

```elisp
;; Add to load-path
(add-to-list 'load-path "/path/to/pjmai-rs/elisp")
(require 'pjmai)
(pjmai-global-mode 1)
```

### With use-package

```elisp
(use-package pjmai
  :load-path "/path/to/pjmai-rs/elisp"
  :config
  (pjmai-global-mode 1))
```

### Custom binary path

```elisp
(setq pjmai-program "/usr/local/bin/pjmai")
```

## Quick Start

1. Enable the mode: `M-x pjmai-global-mode`
2. Switch to a project: `C-c p c` then type a project name (with tab completion)
3. A new shell buffer opens at the project root with correct directory

## Keybindings

All commands are under the `C-c p` prefix:

### Core Commands

| Key | Command | Description |
|-----|---------|-------------|
| `C-c p h` | `pjmai-help` | Show help / alias list |
| `C-c p c` | `pjmai-change` | Switch to project (open shell) |
| `C-c p s` | `pjmai-show` | Show current project |
| `C-c p l` | `pjmai-list` | List all projects |
| `C-c p L` | `pjmai-list-long` | List with extended info |
| `C-c p q` | `pjmai-query` | Check if project exists |
| `C-c p t` | `pjmai-context` | Show project context |
| `C-c p H` | `pjmai-history` | Navigation history |
| `C-c p x` | `pjmai-exports` | Show project exports |
| `C-c p d` | `pjmai-dired` | Open project in dired |

### State-Changing Commands

| Key | Command | Description |
|-----|---------|-------------|
| `C-c p a` | `pjmai-add` | Add a new project |
| `C-c p e` | `pjmai-edit` | Edit project metadata |
| `C-c p r` | `pjmai-remove` | Remove a project |
| `C-c p R` | `pjmai-rename` | Rename a project |
| `C-c p p` | `pjmai-push` | Push to stack and switch |
| `C-c p o` | `pjmai-pop` | Pop from project stack |

### Group Commands (`C-c p g`)

| Key | Command | Description |
|-----|---------|-------------|
| `C-c p g l` | `pjmai-group-list` | List groups |
| `C-c p g s` | `pjmai-group-show` | Show group details |
| `C-c p g p` | `pjmai-group-prompt` | Show group prompt |

### Stack Commands (`C-c p k`)

| Key | Command | Description |
|-----|---------|-------------|
| `C-c p k s` | `pjmai-stack-show` | Show project stack |
| `C-c p k c` | `pjmai-stack-clear` | Clear the stack |

## Common Workflows

### Switch between projects

```
C-c p c → type "myproj" TAB RET
```

This opens (or switches to) a shell buffer `*pjmai:myproj*` at the project root. File completion in that shell works correctly because `default-directory` is set before the shell starts.

### Browse project files

```
C-c p d → type "myproj" TAB RET
```

Opens dired at the project root. From there use standard dired commands.

### Check what project you're in

```
C-c p s
```

Shows current project info in a read-only buffer.

### List all projects with details

```
C-c p L
```

Shows all projects with language, description, and tags.

### Add a project from Emacs

```
C-c p a → "my-new-proj" RET → "/path/to/project/" RET
```

### History navigation

```
C-c p H         → view history
C-u 3 C-c p H   → jump to history entry #3
```

## Customization

### Change the prefix key

```elisp
(setq pjmai-key-prefix "C-c j")  ; Must set before enabling mode
(pjmai-global-mode 1)
```

### Change shell buffer naming

```elisp
(setq pjmai-shell-buffer-format "*proj/%s*")
;; Buffers will be named *proj/myproj* instead of *pjmai:myproj*
```

### Use vterm instead of shell-mode

```elisp
(setq pjmai-shell-function #'vterm)
```

Note: The shell function receives one argument (the buffer name). It must create a buffer with that name.

## Output Buffers

Read-only commands open results in dedicated buffers:

| Buffer | Content |
|--------|---------|
| `*pjmai-help*` | Alias reference |
| `*pjmai-list*` | Project listing |
| `*pjmai-show*` | Current project |
| `*pjmai-history*` | Navigation history |
| `*pjmai-context*` | Project context |
| `*pjmai-exports*` | Export commands |
| `*pjmai-groups*` | Group listing |
| `*pjmai-stack*` | Stack contents |

All use `special-mode`: press `q` to close.

## Running Tests

```bash
cd elisp/
emacs -batch -l ert -l pjmai.el -l pjmai-test.el -f ert-run-tests-batch-and-exit
```

Tests use mock shell scripts and do not require the actual `pjmai` binary.

## Troubleshooting

### "pjmai failed" errors

Check that the binary is installed and accessible:

```bash
which pjmai
pjmai --version
```

Or set the full path:

```elisp
(setq pjmai-program "/full/path/to/pjmai")
```

### Completion shows no projects

Ensure you have projects registered:

```bash
pjmai list
```

If empty, add or scan for projects:

```bash
pjmai scan ~/code
```

### Shell opens in wrong directory

Verify the project path is correct:

```bash
pjmai --json show -p myproject
```

If the path is stale, update it with `pjmai edit`.
