# Design Document

## Design Principles

1. **Emacs knows the directory first** — never let the shell change directories and hope Emacs notices. Resolve the path in Elisp, set `default-directory`, then create the shell.

2. **CLI is the source of truth** — never duplicate project registry logic. All data comes from `pjmai` invocations.

3. **Minimal dependencies** — only requires Emacs 27.1+ (for `json-parse-string` with keyword args). No external packages.

4. **Single-file package** — all code in `pjmai.el` for easy installation (`load-path` or `use-package`).

## CLI Interface Contract

### Plain text (display commands)

```
pjmai aliases          → help text
pjmai list [--long]    → project listing
pjmai show             → current project info
pjmai history [N]      → navigation history
pjmai context [name]   → project context
pjmai exports          → shell export commands
pjmai group list|show|prompt
pjmai stack show|clear
```

### JSON (structured data)

```
pjmai --json show -p NAME  → {"name":"...","path":"...","type":"..."}
pjmai --json list          → {"projects":[...]}
```

### Completion data

```
pjmai complete projects    → newline-separated nicknames
```

### State changes (no display needed)

```
pjmai add NAME -f PATH [--description D] [--language L]
pjmai edit NAME [--description D] [--language L]
pjmai remove NAME
pjmai rename OLD NEW
pjmai query -p NAME        → exit 0 (found) or exit 1 (not found)
```

## Command Categories

### Display Commands

Use `pjmai--display` which:
1. Creates a `get-buffer-create` output buffer
2. Runs `process-file` directly into the buffer
3. Sets `special-mode` (read-only, `q` to quit)
4. Calls `pop-to-buffer`

### Navigation Commands

`pjmai-change` and `pjmai-shell` are the core:
1. Resolve path via `pjmai --json show -p NAME`
2. Check for existing buffer `*pjmai:NAME*`
3. If exists: switch and re-sync directory
4. If new: set `default-directory`, call shell function, rename buffer

### Mutation Commands

`pjmai-add`, `pjmai-edit`, `pjmai-remove`, `pjmai-rename`:
- Call CLI, report result via `message`
- Destructive ops require `yes-or-no-p`

## Keymap Design

```
C-c p           ← prefix (user-reserved namespace)
├── h           help/aliases
├── c           change (open project shell)
├── s           show current
├── l           list
├── L           list --long
├── q           query
├── t           context
├── x           exports
├── H           history
├── d           dired at project root
├── a           add
├── e           edit
├── r           remove
├── R           rename
├── p           push
├── o           pop
├── g           group submap
│   ├── l       group list
│   ├── s       group show
│   └── p       group prompt
└── k           stack submap
    ├── s       stack show
    └── c       stack clear
```

Lowercase for frequent commands, uppercase for less common. Submaps for related groups.

## Shell Buffer Lifecycle

```
                  ┌─────────────┐
                  │  User calls  │
                  │ pjmai-change │
                  └──────┬──────┘
                         │
                  ┌──────▼──────┐
                  │ Resolve path │
                  │ via --json   │
                  └──────┬──────┘
                         │
                ┌────────▼────────┐
                │ Buffer exists?  │
                └───┬─────────┬───┘
                YES │         │ NO
          ┌─────────▼───┐ ┌──▼──────────────┐
          │ pop-to-buf   │ │ default-dir=path│
          │ set dir      │ │ shell(bufname)  │
          │ send cd      │ │ rename buffer   │
          └──────────────┘ │ pop-to-buffer   │
                           └─────────────────┘
```

## Testing Strategy

- **Mock-based**: tests create temporary shell scripts that simulate CLI output
- **No real CLI needed**: tests run in CI without the Rust binary
- **Categories tested**:
  - Core CLI call (success, failure, args, empty output)
  - JSON parsing (objects, arrays)
  - Project name completion (split, empty)
  - Path resolution (valid directory, invalid path)
  - Buffer naming (default format, custom format)
  - Display buffer creation (content, mode, error)
  - Keymap structure (all bindings, submaps)
  - Global mode (enable/disable)
  - Customization defaults

## Future Extension Points

- **`pjmai-shell-function`**: swap shell for vterm/eshell
- **`pjmai-shell-buffer-format`**: customize naming scheme
- **`pjmai-key-prefix`**: change from `C-c p` to any prefix
- **Post-shell hooks**: source setup files, activate envs
- **`project.el` backend**: register pjmai projects as Emacs projects
