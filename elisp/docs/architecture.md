# Architecture

## Overview

The pjmai.el package provides Emacs-native integration with the pjmai-rs Rust CLI. Rather than wrapping shell aliases (which cause stale `default-directory` in shell-mode), it calls the binary directly and manages Emacs state explicitly.

## Two-Layer Design

```
┌─────────────────────────────────┐
│         Emacs (pjmai.el)        │
│                                 │
│  ┌───────────┐  ┌────────────┐  │
│  │  Keymap    │  │  Shell Buf │  │
│  │  C-c p *  │  │  *pjmai:x* │  │
│  └─────┬─────┘  └──────┬─────┘  │
│        │               │        │
│  ┌─────▼───────────────▼─────┐  │
│  │   Core CLI Interface      │  │
│  │  pjmai--call / --call-json│  │
│  └───────────┬───────────────┘  │
└──────────────┼──────────────────┘
               │ process-file
┌──────────────▼──────────────────┐
│      pjmai binary (Rust)        │
│  ┌──────────┐  ┌─────────────┐  │
│  │ Resolver  │  │ JSON Output │  │
│  │ nickname  │  │ --json flag │  │
│  │  → path   │  │  → plist   │  │
│  └──────────┘  └─────────────┘  │
└─────────────────────────────────┘
```

## Key Architectural Decisions

### 1. Direct binary invocation, not shell aliases

Shell aliases (`chpj`, `lspj`) are zsh/bash functions that only work in interactive shells. Emacs `process-file` / `call-process` cannot expand them. The package calls `pjmai` directly with subcommand arguments.

### 2. Per-project shell buffers

Instead of one shell where `cd` might get out of sync, each project gets a dedicated buffer (`*pjmai:projectname*`). `default-directory` is set **before** the shell starts, so Emacs completion works immediately.

### 3. JSON for structured data, plain text for display

- `pjmai--call-json` uses `--json` flag for machine-readable data (resolve paths, parse project metadata)
- `pjmai--display` uses plain text output in read-only `special-mode` buffers

### 4. Sparse keymap under a single prefix

All commands live under `C-c p` (user-reserved namespace). Submaps (`g` for groups, `k` for stack) keep the namespace flat and mnemonic.

### 5. Pluggable shell function

`pjmai-shell-function` defaults to `#'shell` but can be set to `#'vterm` or a custom function for alternative terminal emulators.

## Module Structure

```
elisp/
├── pjmai.el          # Main package (single file)
│   ├── Customization   defgroup, defcustom
│   ├── Core CLI        pjmai--call, pjmai--call-json, pjmai--display
│   ├── Completion      pjmai--project-names, pjmai--read-project
│   ├── Resolution      pjmai-resolve-path
│   ├── Shell mgmt      pjmai-shell, pjmai--shell-buffer-name
│   ├── Commands        pjmai-{help,list,show,change,query,...}
│   ├── Keymap          pjmai-command-map + submaps
│   └── Minor mode      pjmai-global-mode
├── pjmai-test.el     # ERT test suite
└── docs/
    ├── architecture.md
    ├── prd.md
    ├── design.md
    ├── plan.md
    ├── status.md
    └── usage.md
```

## Data Flow: Project Change

```
User: C-c p c → "myproj"
  │
  ▼
pjmai-change("myproj")
  │
  ▼
pjmai-shell("myproj")
  │
  ├── pjmai-resolve-path("myproj")
  │     └── pjmai --json show -p myproj
  │           → {"name":"myproj","path":"/home/user/myproj","type":"directory"}
  │           → "/home/user/myproj/"
  │
  ├── Buffer *pjmai:myproj* exists?
  │     ├── YES → pop-to-buffer, re-sync default-directory, send cd
  │     └── NO  → set default-directory, create shell, rename buffer
  │
  └── User is now in shell at /home/user/myproj/
      with correct default-directory for completion
```

## Error Handling

- Non-zero exit from `pjmai` → `error` signal with CLI output
- Non-existent resolved path → `error` before shell creation
- Destructive commands (remove, stack clear) → `yes-or-no-p` confirmation in Emacs
