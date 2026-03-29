# CHANGES

## 2026-03-29

- Add Emacs integration package (`elisp/pjmai.el`) with per-project shell buffers
- ERT test suite (`elisp/pjmai-test.el`) with 25 tests using mock CLI
- Prefix keymap `C-c p` with mnemonic bindings for all pjmai commands
- Project name tab completion via `completing-read`
- Resolve project paths via `pjmai-rs --json change -p NAME`
- Handle pjmai exit codes 0-3 as success (shell signaling protocol)
- Auto-discover binary at `~/.local/bin/pjmai-rs`
- Elisp docs: architecture, PRD, design, plan, status, usage

## 2026-03-07

- Add project navigation history with `hypj` alias (`pjmai history`)
- Add `pjmai stack show|clear` subcommand with `stpj` alias
- Clear push/pop stack on `chpj` (non-stack navigation abandons stack workflow)
- Show project name and remaining depth on `popj`
- Clear project stack during `source update.sh`
- Update help text for change, pop, push, stack commands
- Add `rmpj --all` to remove all projects with confirmation prompt
- Add `scpj --reset` for fresh re-scan (clears registry then scans)
- Improve scan nickname collisions: use owner-prefixed names instead of numeric suffixes
- `stpj` defaults to show (no subcommand required), `stpj clear` prompts for confirmation
- Add user guide (`docs/user-guide.md`) with all commands, scenarios, and examples
- Fuzzy tab completion: prefix matches first, then segment matches (after `-`), then substring matches
- Tab completion results sorted by most recently used within each tier

## 2026-03-06

- Add subdirectory navigation with tab completion
- Document update.sh development workflow

## 2026-03-04

- Rename binary from pjmai to pjmai-rs to match repo name
- Replace wrightmikea references with generic examples
- Add -V/--version flag with build metadata
- Add process docs, copyright, license
- Document --yes/-y flag in README and CLAUDE.md
- Add --yes/-y flag for non-interactive/scripted use
- Update paths for repository rename to sw-cli-tools/pjmai-rs
- Add project groups feature with auto-inference from directory structure
- Add comprehensive ELI5 documentation
- Add environment auto-detection for projects (Phase 3.2)
- Add on_exit hooks, path_prepend, and path_remove for environment config (Phase 3.1)
- Rename to pjmai-rs, add evpj alias, update docs
- Add per-project environment configuration (Phase 3)

## 2026-03-03

- Add config export/import with tests and documentation
- Add secure .pjmai.sh approval workflow with hash verification
- Add srcpj for explicit opt-in project environment sourcing
- Fix install.sh prompt check to handle colored output
- Add --prompt flag to install.sh and setup command for shell prompt integration
- Add push/pop stack navigation for project switching
- Add scan command to discover git repositories
- Add rename command (mvpj) for changing project nicknames
- Fix config prompt not visible when stdout captured
- Add uninstall.sh script for clean removal
- Implement debug flag (-d) for troubleshooting
- Add fast tab completion with pjmai complete command
- Add install.sh for one-line installation
- Add setup command for automated shell integration
- Add --json flag for machine-readable output
- Fix clippy warnings in integration tests
- Rename project from pjm1 to pjmai
- Add comprehensive improvements roadmap document

## 2026-01-28

- Upgrade to Rust 2024 edition and fix critical remove() bug
- Add dependency injection, tests, documentation, and VHS demos
- Add proper error handling, shell completions, and fuzzy matching
- Fix shell integration to work with both bash and zsh
- Convert shell aliases to functions and add tab completion

## 2021-09-28

- Require doc and disallow warnings; add doc
- Use type aliases instead of String in some cases
- Add logging

## 2021-09-09

- Add prompt command

## 2021-08-05

- Use generated version

## 2021-05-22

- Add panic when adding duplicate project name

## 2020-03-03

- Refactor to fix clippy warnings

## 2020-02-29

- Add shpj subcommand; colorize lspj/shpj output

## 2020-02-26

- Add build script to generate timestamped version
- Format/sort lspj output; convert $HOME prefix to ~

## 2020-02-24

- Working proof of concept: adpj, chpj, hlpj, lspj, and rmpj

## 2020-02-22

- Initial project skeleton
