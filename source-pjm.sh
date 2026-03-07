#!/bin/sh
# Shell integration for PJMAI - provides wrapper function and command functions
# Works with both bash and zsh in interactive and non-interactive modes

# Approval file for trusted .pjmai.sh files (hash:path format)
_PJMAI_APPROVALS="${PJMAI_CONFIG_DIR:-$HOME/.pjmai}/approved-envs"

# Compute hash of a file (portable across macOS/Linux)
_pjm_hash() {
    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum "$1" 2>/dev/null | cut -d' ' -f1
    elif command -v shasum >/dev/null 2>&1; then
        shasum -a 256 "$1" 2>/dev/null | cut -d' ' -f1
    else
        # Fallback: use file size + mtime (less secure but works)
        stat -f "%z:%m" "$1" 2>/dev/null || stat -c "%s:%Y" "$1" 2>/dev/null
    fi
}

# Check if .pjmai.sh is approved (hash matches)
_pjm_is_approved() {
    local file="$1"
    local current_hash=$(_pjm_hash "$file")
    local full_path=$(cd "$(dirname "$file")" && pwd)/$(basename "$file")

    [[ -f "$_PJMAI_APPROVALS" ]] && grep -q "^${current_hash}:${full_path}$" "$_PJMAI_APPROVALS" 2>/dev/null
}

# Core wrapper that handles exit codes for directory changes and file sourcing
pjm_fn() {
    # Reset zsh to default options - critical because pjmai-rs uses non-zero exit codes
    # intentionally (2=cd, 3=source, 5=eval) and errexit would kill the shell
    [[ -n "$ZSH_VERSION" ]] && emulate -L zsh

    # For bash, save and disable errexit (will restore at end)
    local _pjm_errexit_was_set=""
    if [[ -n "$BASH_VERSION" ]]; then
        [[ $- == *e* ]] && _pjm_errexit_was_set=1
        set +e
    fi

    # Debug mode - set _PJM_DEBUG=1 to trace execution
    [[ -n "$_PJM_DEBUG" ]] && echo "[pjm_fn] args: $*" >&2

    # Run on_exit from previous project first (if any)
    if [[ -n "$_PJMAI_ON_EXIT" ]]; then
        [[ -n "$_PJM_DEBUG" ]] && echo "[pjm_fn] running on_exit: $_PJMAI_ON_EXIT" >&2
        eval "$_PJMAI_ON_EXIT" || true
        _PJMAI_ON_EXIT=""
    fi

    PJM_OUT=$(pjmai-rs "$@")
    PJM_EXIT=$?
    [[ -n "$_PJM_DEBUG" ]] && echo "[pjm_fn] exit=$PJM_EXIT out='$PJM_OUT'" >&2

    case "$PJM_EXIT" in
        2)
            if [[ -z "$PJM_OUT" ]]; then
                echo "Error: empty path returned"
                return 1
            fi
            [[ -n "$_PJM_DEBUG" ]] && echo "[pjm_fn] cd to: $PJM_OUT" >&2
            cd "${PJM_OUT}" || { echo "Failed to cd to: ${PJM_OUT}"; return 1; }
            [[ -n "$_PJM_DEBUG" ]] && echo "[pjm_fn] cd succeeded, checking .pjmai.sh" >&2
            # Check for .pjmai.sh
            if [[ -f .pjmai.sh ]]; then
                [[ -n "$_PJM_DEBUG" ]] && echo "[pjm_fn] found .pjmai.sh" >&2
                if _pjm_is_approved .pjmai.sh; then
                    [[ -n "$_PJM_DEBUG" ]] && echo "[pjm_fn] sourcing approved .pjmai.sh" >&2
                    source .pjmai.sh
                else
                    # New or changed - warn
                    echo -e "\033[0;33mFound .pjmai.sh\033[0m - inspect: 'cat .pjmai.sh', approve: 'srcpj'"
                fi
            fi
            [[ -n "$_PJM_DEBUG" ]] && echo "[pjm_fn] case 2 done" >&2
            ;;
        3)
            source "$PJM_OUT" || { echo "Failed to source: ${PJM_OUT}"; return 1; }
            ;;
        5)
            # Execute environment setup script (cd + exports + on_enter)
            eval "$PJM_OUT" || { echo "Failed to execute environment setup"; return 1; }
            # Check for .pjmai.sh after environment setup
            if [[ -f .pjmai.sh ]]; then
                if _pjm_is_approved .pjmai.sh; then
                    source .pjmai.sh
                else
                    echo "\033[0;33mFound .pjmai.sh\033[0m - inspect: 'cat .pjmai.sh', approve: 'srcpj'"
                fi
            fi
            ;;
        *)
            echo "$PJM_OUT"
            ;;
    esac

    # Restore bash errexit if it was set
    [[ -n "$BASH_VERSION" && -n "$_pjm_errexit_was_set" ]] && set -e

    [[ -n "$_PJM_DEBUG" ]] && echo "[pjm_fn] function complete" >&2
}

# Source .pjmai.sh and approve it (explicit opt-in for project environment)
srcpj() {
    if [[ -f .pjmai.sh ]]; then
        local current_hash=$(_pjm_hash .pjmai.sh)
        local full_path=$(pwd)/.pjmai.sh

        # Source the file
        echo "Sourcing .pjmai.sh..."
        source .pjmai.sh

        # Remove old approval for this path (if any)
        if [[ -f "$_PJMAI_APPROVALS" ]]; then
            grep -v ":${full_path}$" "$_PJMAI_APPROVALS" > "${_PJMAI_APPROVALS}.tmp" 2>/dev/null || true
            mv "${_PJMAI_APPROVALS}.tmp" "$_PJMAI_APPROVALS"
        fi

        # Add new approval
        mkdir -p "$(dirname "$_PJMAI_APPROVALS")"
        echo "${current_hash}:${full_path}" >> "$_PJMAI_APPROVALS"
        echo "\033[0;32mApproved\033[0m - will auto-source until file changes"
    else
        echo "No .pjmai.sh in current directory"
    fi
}

# Command functions (work in both interactive and non-interactive shells)
adpj() { pjm_fn add -p "$@"; }
chpj() { pjm_fn change -p "$@"; }
ctpj() { pjm_fn context "$@"; }
evpj() { pjm_fn env -p "$@"; }
hlpj() { pjm_fn aliases "$@"; }
lspj() { pjm_fn list "$@"; }
mvpj() { pjm_fn rename -f "$1" -t "$2"; }
popj() { pjm_fn pop "$@"; }
prpj() { pjm_fn prompt "$@"; }
pspj() { pjm_fn push -p "$@"; }
rmpj() { pjm_fn remove --project "$@"; }
scpj() { pjm_fn scan "$@"; }
shpj() { pjm_fn show "$@"; }

# Group command functions
lsgp() { pjm_fn group list "$@"; }
shgp() { pjm_fn group show "$@"; }
prgp() { pjm_fn group prompt "$@"; }

# Helper to get project names for completion (uses fast native completion)
_pjm_projects() {
    pjmai-rs complete projects 2>/dev/null
}

# Shell-specific completion setup
if [ -n "$ZSH_VERSION" ]; then
    # Zsh completion for chpj with subdirectory support
    _pjm_chpj_complete() {
        if [[ ${CURRENT} -eq 2 ]]; then
            # First argument - complete project names
            local projects
            projects=(${(f)"$(pjmai-rs complete projects "${words[2]}" 2>/dev/null)"})
            _describe 'project' projects
        else
            # Subsequent arguments - complete subdirs within project
            local project="${words[2]}"
            local subdirs
            subdirs=(${(f)"$(pjmai-rs complete subdirs "$project" "${words[@]:2}" 2>/dev/null)"})
            _describe 'subdir' subdirs
        fi
    }
    compdef _pjm_chpj_complete chpj

    # Zsh completion for other commands (no subdir support)
    _pjm_complete() {
        local projects
        projects=(${(f)"$(pjmai-rs complete projects "${words[CURRENT]}" 2>/dev/null)"})
        _describe 'project' projects
    }
    compdef _pjm_complete rmpj pspj

elif [ -n "$BASH_VERSION" ]; then
    # Bash completion for chpj with subdirectory support
    _pjm_chpj_complete() {
        local cur="${COMP_WORDS[COMP_CWORD]}"
        if [[ ${COMP_CWORD} -eq 1 ]]; then
            # First argument - complete project names
            COMPREPLY=($(pjmai-rs complete projects "$cur" 2>/dev/null))
        else
            # Subsequent arguments - complete subdirs within project
            local project="${COMP_WORDS[1]}"
            COMPREPLY=($(pjmai-rs complete subdirs "$project" "${COMP_WORDS[@]:2}" 2>/dev/null))
        fi
    }
    complete -F _pjm_chpj_complete chpj

    # Bash completion for other commands (no subdir support)
    _pjm_complete() {
        local cur="${COMP_WORDS[COMP_CWORD]}"
        COMPREPLY=($(pjmai-rs complete projects "$cur" 2>/dev/null))
    }
    complete -F _pjm_complete rmpj pspj
fi
