#!/bin/sh
# Shell integration for PJMAI - provides wrapper function and command functions
# Works with both bash and zsh in interactive and non-interactive modes

# Core wrapper that handles exit codes for directory changes and file sourcing
pjm_fn() {
    PJM_OUT=$(pjmai "$@")
    PJM_EXIT=$?
    case "$PJM_EXIT" in
        2)
            cd "${PJM_OUT}"
            ;;
        3)
            source "$PJM_OUT"
            ;;
        *)
            echo "$PJM_OUT"
            ;;
    esac
}

# Command functions (work in both interactive and non-interactive shells)
adpj() { pjm_fn add -p "$@"; }
chpj() { pjm_fn change -p "$@"; }
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
