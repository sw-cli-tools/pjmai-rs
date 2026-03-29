# Implementation Plan

## Phase 1: Core (Complete)

**Goal**: Working project switching with correct `default-directory`.

- [x] Core CLI interface (`pjmai--call`, `pjmai--call-json`)
- [x] Display command helper (`pjmai--display`)
- [x] Project name completion (`pjmai--project-names`, `pjmai--read-project`)
- [x] Path resolution (`pjmai-resolve-path`)
- [x] Per-project shell buffers (`pjmai-shell`)
- [x] Read-only commands: help, list, show
- [x] Change command (opens project shell)
- [x] Query command
- [x] Prefix keymap (`C-c p`)
- [x] Global minor mode
- [x] ERT test suite with mock CLI

## Phase 2: Full Command Coverage

**Goal**: All pjmai commands accessible from Emacs.

- [x] History (with prefix arg for jump)
- [x] Context (optional project name)
- [x] Exports
- [x] Add, edit, remove, rename
- [x] Push, pop
- [x] Group submap (list, show, prompt)
- [x] Stack submap (show, clear)
- [x] Dired integration
- [x] List long variant

## Phase 3: Enhanced UX

**Goal**: Discoverability and polish.

- [ ] `which-key` integration (automatic if user has which-key installed)
- [ ] `transient` menu (Magit-style command panel)
- [ ] Minibuffer annotations for project completion (path, language, group)
- [ ] Modeline indicator showing current project name
- [ ] Scan command with progress reporting

## Phase 4: Deep Emacs Integration

**Goal**: pjmai feels like a native Emacs feature.

- [ ] `project.el` backend (register pjmai projects as Emacs project roots)
- [ ] Project-local compile commands via pjmai context
- [ ] Setup file sourcing after shell creation
- [ ] Env config display/edit from Emacs
- [ ] `vterm` shell function support (tested)
- [ ] `eshell` shell function support
- [ ] Integration with `rg` / `grep` for project-scoped search
- [ ] Dired bookmark for project roots

## Phase 5: Package Distribution

**Goal**: Easy installation for others.

- [ ] MELPA-compatible package structure
- [ ] Autoload cookies on all interactive commands
- [ ] `use-package` configuration example
- [ ] Byte-compilation clean
- [ ] CI: run ERT tests on Emacs 27.1, 28, 29, 30

## Dependencies

| Phase | Depends On | Notes |
|-------|-----------|-------|
| 1 | pjmai binary installed | `pjmai-program` must be in PATH |
| 2 | Phase 1 | Extends command set |
| 3 | Phase 2 | UX layer on complete commands |
| 4 | Phase 2 | Deep hooks need full command coverage |
| 5 | Phase 3+ | Package quality gate |

## Risk Mitigation

| Risk | Mitigation |
|------|-----------|
| CLI output format changes | Use `--json` for structured data; plain text only for display |
| Shell function compatibility | `pjmai-shell-function` customization point |
| Keymap conflicts with other packages | `C-c p` is user-reserved; prefix is configurable |
| Emacs version compatibility | Only require 27.1+ for `json-parse-string` |
