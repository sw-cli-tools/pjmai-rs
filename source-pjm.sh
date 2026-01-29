#!/bin/sh
# Shell integration for PJM1 - provides wrapper function and command functions
# Works with both bash and zsh in interactive and non-interactive modes

# Core wrapper that handles exit codes for directory changes and file sourcing
pjm_fn() {
    PJM_OUT=$(pjm1 "$@")
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
prpj() { pjm_fn prompt "$@"; }
rmpj() { pjm_fn remove --project "$@"; }
shpj() { pjm_fn show "$@"; }

# Helper to get project names for completion
_pjm_projects() {
    pjm1 list 2>/dev/null | sed 's/^[> ]//' | awk '{print $1}'
}

# Shell-specific completion setup
if [ -n "$ZSH_VERSION" ]; then
    # Zsh completion
    _pjm_complete() {
        local projects
        projects=(${(f)"$(_pjm_projects)"})
        _describe 'project' projects
    }
    compdef _pjm_complete chpj rmpj
elif [ -n "$BASH_VERSION" ]; then
    # Bash completion
    _pjm_complete() {
        local cur="${COMP_WORDS[COMP_CWORD]}"
        COMPREPLY=($(compgen -W "$(_pjm_projects)" -- "$cur"))
    }
    complete -F _pjm_complete chpj rmpj
fi
