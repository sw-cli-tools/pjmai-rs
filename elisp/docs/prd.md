# Product Requirements Document

## Problem Statement

When using pjmai-rs shell aliases inside Emacs `shell-mode`, directory changes made by the Rust CLI (via exit code signaling to the parent shell) do not update Emacs's `default-directory`. This causes:

- Stale file completion (completing from the old directory)
- Incorrect `default-directory` for `find-file`, `compile`, and other commands
- Manual workaround required (`M-x dirs` after every project switch)

## Goal

Provide an Emacs-native integration layer for pjmai-rs that eliminates the stale directory problem by managing shell buffers and directory state directly from Elisp.

## Target Users

Emacs users who:
- Use pjmai-rs for project management in terminal
- Run shells inside Emacs (shell-mode, vterm, or eshell)
- Want project switching to "just work" without manual resync

## Requirements

### Must Have (P0)

| ID | Requirement |
|----|-------------|
| R1 | Call `pjmai` binary directly (not shell aliases) |
| R2 | Resolve project nickname → absolute path via CLI |
| R3 | Create/reuse per-project named shell buffers (`*pjmai:name*`) |
| R4 | Set `default-directory` correctly before shell creation |
| R5 | Provide prefix keymap (`C-c p`) with mnemonic subcommands |
| R6 | Support all read-only commands: help, list, show, history, context, exports |
| R7 | Support project change via named shell buffer |
| R8 | Tab completion for project nicknames via `completing-read` |

### Should Have (P1)

| ID | Requirement |
|----|-------------|
| R9 | Re-sync existing shell buffer when switching back to a project |
| R10 | Support add, edit, remove, rename commands |
| R11 | Group and stack submaps |
| R12 | Dired integration (open project root in dired) |
| R13 | Configurable shell function (shell, vterm, eshell) |
| R14 | Global minor mode for easy enable/disable |

### Nice to Have (P2)

| ID | Requirement |
|----|-------------|
| R15 | Integration with `project.el` |
| R16 | `which-key` / `transient` discoverability |
| R17 | Project-local compile commands |
| R18 | Setup file sourcing after shell creation |
| R19 | Env config display/management from Emacs |

## Non-Requirements

- This package does NOT replace shell aliases for terminal use
- This package does NOT modify the Rust CLI
- This package does NOT require specific Emacs packages beyond core

## Success Criteria

1. `C-c p c myproj RET` opens a shell at the correct directory with working completion
2. Switching back to an existing project shell reuses the buffer
3. All pjmai read-only commands accessible via prefix keymap
4. ERT test suite passes with mock CLI
