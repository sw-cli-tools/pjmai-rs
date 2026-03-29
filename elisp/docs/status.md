# Status

## Current: v0.1.0 — Phase 1+2 Complete

**Date**: 2026-03-28

### What's Done

| Component | Status | Notes |
|-----------|--------|-------|
| Core CLI interface | Done | `pjmai--call`, `pjmai--call-json`, `pjmai--display` |
| Project completion | Done | `completing-read` with `pjmai complete projects` |
| Path resolution | Done | Via `pjmai --json show -p NAME` |
| Shell buffer mgmt | Done | Create, reuse, re-sync `default-directory` |
| Display commands | Done | help, list, list-long, show, history, context, exports |
| Navigation commands | Done | change (→ shell), push, pop |
| Mutation commands | Done | add, edit, remove, rename |
| Group commands | Done | list, show, prompt (under `g` submap) |
| Stack commands | Done | show, clear (under `k` submap) |
| Dired integration | Done | `C-c p d` opens project root |
| Query command | Done | Reports exist/not-found via message |
| Keymap | Done | Full `C-c p` prefix with all bindings |
| Global minor mode | Done | `pjmai-global-mode` with lighter " pjm" |
| ERT tests | Done | 22 tests covering core, keymap, mode, customization |

### What's Next

- Phase 3: `which-key` labels, `transient` menu, completion annotations
- Phase 4: `project.el` backend, setup file sourcing, vterm support

### Known Limitations

1. **No setup file sourcing yet** — projects with `.pjmai.sh` env files won't auto-source in new shells
2. **Shell function untested with vterm** — `pjmai-shell-function` is configurable but only `#'shell` is tested
3. **No modeline indicator** — current project not shown in modeline
4. **Completion is unadorned** — no annotations showing path/language/group alongside nicknames

### Files

```
elisp/
├── pjmai.el           274 lines  Main package
├── pjmai-test.el      196 lines  ERT test suite (22 tests)
└── docs/
    ├── architecture.md            System design and data flow
    ├── prd.md                     Product requirements
    ├── design.md                  Detailed design decisions
    ├── plan.md                    Phased implementation plan
    ├── status.md                  This file
    ├── usage.md                   User guide and configuration
    └── research.txt               Original research and analysis
```
