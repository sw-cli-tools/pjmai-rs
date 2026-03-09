# Shell Support

## Currently Supported

### Zsh (primary)
- Full support: aliases, tab completion (fuzzy/substring/case-insensitive), named directories via `xppj`
- Completion uses `compadd` with matchers for case-insensitive + substring matching
- Named directories (`hash -d`) work natively with `~nickname/path` syntax and tab completion at every level
- macOS default shell since Catalina (10.15)

### Bash (full support)
- Full support: aliases, tab completion (prefix + case-insensitive via Rust binary)
- Completion uses `complete -F` with `COMPREPLY`
- No named directory equivalent; `xppj --format bash` exports `PJMAI_*` environment variables instead
- Default shell on most Linux distributions (Arch, Ubuntu, Fedora, Debian)
- Hyphens in project names become underscores in env var names (e.g., `pjmai-rs` -> `$PJMAI_PJMAI_RS`)

## Partially Supported

### Fish
- `clap_complete` can generate fish completions (`pjmai completions fish`)
- `xppj --format fish` exports `PJMAI_*` universal variables
- No `source-pjm.sh` equivalent yet (fish uses different function/alias syntax)
- Fish functions use `function name; ...; end` instead of `name() { ...; }`
- Exit code handling would need fish's `status` variable instead of `$?`

## Not Yet Supported

### Shells to Consider

| Shell | Popularity | Effort | Value | Notes |
|-------|-----------|--------|-------|-------|
| **Fish** | High (growing) | Medium | High | Different syntax, no POSIX compat, but large modern user base |
| **PowerShell** | Medium | High | Medium | Windows/cross-platform, very different paradigm |
| **Elvish** | Low | Medium | Low | Modern, clap_complete support exists |
| **Nushell** | Low (growing) | High | Low | Structured data shell, interesting fit but niche |
| **Ksh** | Low | Low | Low | POSIX-compatible, `source-pjm.sh` likely works as-is |
| **Tcsh/Csh** | Very Low | High | Very Low | Non-POSIX, different syntax (`setenv`, `alias`), legacy only |
| **Bourne Shell (sh)** | N/A | Low | Low | `source-pjm.sh` already uses `#!/bin/sh` header but requires `[[` → `[` changes |
| **Eshell** | Low | High | Low | Emacs built-in shell; runs inside Emacs, uses Elisp for config |
| **Vterm/Term** | Low | None | Free | Emacs terminal emulators that run bash/zsh inside them (like Warp) |
| **Warp** | Medium | None | Free | Warp is a terminal emulator, not a shell. It runs zsh/bash/fish inside it, so pjmai-rs already works in Warp |

### What "Warp" Actually Is

Warp is a GPU-accelerated terminal emulator (like iTerm2 or Alacritty), not a shell. It runs your chosen shell (zsh, bash, fish) inside it. Since pjmai-rs integrates at the shell level, it works in Warp automatically with no changes needed. Warp's AI features and block-based UI are terminal-level, not shell-level.

### Emacs Shell Environments

Emacs has several ways to run shells, each with different implications:

- **Vterm / Term / Ansi-term**: These are terminal emulators inside Emacs. They run your real shell (bash/zsh) as a subprocess. pjmai-rs works in these with no changes — just like running in iTerm2 or Warp.

- **Eshell**: A shell implemented entirely in Emacs Lisp. It's not bash or zsh — it's its own shell with Elisp syntax for scripting. Supporting eshell would require:
  - An Elisp integration file instead of `source-pjm.sh`
  - Eshell aliases use `(eshell/alias "name" "command $*")` syntax
  - Directory changes use `(cd path)` in Elisp
  - No standard completion API — uses Emacs's `pcomplete` framework
  - Exit code handling would need Elisp wrappers around process calls

  **Verdict**: High effort, low value. Eshell users who need project switching can use Emacs's built-in `project.el` or `projectile.el` instead. Users who want pjmai-rs specifically should use vterm, which gives them full zsh/bash support inside Emacs.

- **Shell-mode (M-x shell)**: Runs a real shell but with limited terminal capabilities. `source-pjm.sh` can work here but tab completion may not function correctly since shell-mode doesn't fully support `compdef`/`complete`. Basic alias usage (chpj, lspj, etc.) works.

## Challenges of Multi-Shell Support

### 1. Syntax Divergence
- **POSIX-family** (bash, zsh, ksh, sh): Share most syntax. `source-pjm.sh` works across these with minor tweaks.
- **Fish**: Completely different syntax. No `$?`, no `$(...)`, no `[[`. Needs a separate `source-pjm.fish`.
- **Csh-family** (tcsh, csh): `setenv` instead of `export`, `alias` instead of functions. Would need `source-pjm.csh`.
- **PowerShell**: `.ps1` scripts, cmdlet-style aliases. Would need `source-pjm.ps1`.

### 2. Completion Systems
Each shell has a different completion API:
- **Zsh**: `compdef` + `compadd` with rich matcher syntax
- **Bash**: `complete -F` with `COMPREPLY` array
- **Fish**: `complete -c command -a '(helper)'`
- **PowerShell**: `Register-ArgumentCompleter`

`clap_complete` handles basic completions for all of these, but our custom fuzzy/subdir completion requires shell-specific code.

### 3. Exit Code Handling
The core integration (exit codes 2/3/4/5) is portable across POSIX shells but needs translation for fish and PowerShell.

### 4. Named Directories
- **Zsh**: Native `hash -d` with `~name/path` syntax and tab completion
- **Bash**: No equivalent. Must use `$VAR/path` syntax (no tab completion into the path)
- **Fish**: No equivalent. Can use `$VAR/path` with `set -gx`
- **PowerShell**: Can use `$env:VAR` or PowerShell drives

### 5. Testing Burden
Each supported shell multiplies the test matrix. Currently testing bash + zsh is manageable. Adding fish would ~1.5x the shell integration testing effort.

## Recommended Roadmap

1. **Current**: Bash + Zsh (covers ~95% of developer machines)
2. **Next**: Fish (via `source-pjm.fish` — growing user base, especially among newer developers)
3. **Maybe**: PowerShell (for Windows users who don't use WSL)
4. **Unlikely**: Csh/Tcsh (legacy, declining use, not worth the maintenance cost)

## Installation on Linux (Arch, Ubuntu, etc.)

pjmai-rs works on Linux with bash out of the box:

```bash
# Install (requires git + cargo)
curl -fsSL https://raw.githubusercontent.com/sw-cli-tools/pjmai-rs/main/install.sh | bash

# Or build from source
git clone https://github.com/sw-cli-tools/pjmai-rs.git
cd pjmai-rs
cargo build --release
cp target/release/pjmai-rs ~/.local/bin/

# Setup shell integration
pjmai setup bash   # or: pjmai setup zsh
```

The install script auto-detects the shell and configures the appropriate rc file (`.bashrc` for bash, `.zshrc` for zsh).

### Arch Linux Specifics
- Bash is the default shell on Arch
- `~/.local/bin` is in `$PATH` by default on most Arch setups
- The `source-pjm.sh` script is POSIX-compatible and works with bash 5.x (Arch's default)
- Tab completion works via bash's `complete` builtin
- Named directories (via `xppj --format bash`) use `PJMAI_*` env vars instead of zsh's `hash -d`
