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
    PJM_OUT=$(pjmai "$@")
    PJM_EXIT=$?
    case "$PJM_EXIT" in
        2)
            cd "${PJM_OUT}"
            # Check for .pjmai.sh
            if [[ -f .pjmai.sh ]]; then
                if _pjm_is_approved .pjmai.sh; then
                    # Approved and unchanged - auto-source
                    source .pjmai.sh
                else
                    # New or changed - warn
                    echo "\033[0;33mFound .pjmai.sh\033[0m - inspect: 'cat .pjmai.sh', approve: 'srcpj'"
                fi
            fi
            ;;
        3)
            source "$PJM_OUT"
            ;;
        5)
            # Execute environment setup script (cd + exports + on_enter)
            eval "$PJM_OUT"
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
hlpj() { pjm_fn aliases "$@"; }
lspj() { pjm_fn list "$@"; }
mvpj() { pjm_fn rename -f "$1" -t "$2"; }
popj() { pjm_fn pop "$@"; }
prpj() { pjm_fn prompt "$@"; }
pspj() { pjm_fn push -p "$@"; }
rmpj() { pjm_fn remove --project "$@"; }
scpj() { pjm_fn scan "$@"; }
shpj() { pjm_fn show "$@"; }

# Helper to get project names for completion (uses fast native completion)
_pjm_projects() {
    pjmai complete projects 2>/dev/null
}

# Shell-specific completion setup
if [ -n "$ZSH_VERSION" ]; then
    # Zsh completion
    _pjm_complete() {
        local projects
        # Use prefix filtering for faster completion with many projects
        projects=(${(f)"$(pjmai complete projects "${words[CURRENT]}" 2>/dev/null)"})
        _describe 'project' projects
    }
    compdef _pjm_complete chpj rmpj pspj
elif [ -n "$BASH_VERSION" ]; then
    # Bash completion with prefix filtering
    _pjm_complete() {
        local cur="${COMP_WORDS[COMP_CWORD]}"
        # Pass prefix directly to pjmai for fast filtering
        COMPREPLY=($(pjmai complete projects "$cur" 2>/dev/null))
    }
    complete -F _pjm_complete chpj rmpj pspj
fi
