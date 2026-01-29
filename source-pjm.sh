#!/bin/sh
# Shell integration for PJM1 - provides wrapper function and aliases
# Works with both bash and zsh
function pjm_fn() {
    # Use "$@" directly to preserve argument splitting in both bash and zsh
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
alias adpj='pjm_fn add -p'
alias chpj='pjm_fn change -p'
alias hlpj='pjm_fn aliases'
alias lspj='pjm_fn list'
alias prpj='pjm_fn prompt'
alias rmpj='pjm_fn remove --project'
alias shpj='pjm_fn show'
