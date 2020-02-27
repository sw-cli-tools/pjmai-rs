#!/bin/sh
function pjm_fn() {
    PJM_OUT=`pjm1 $@`
    case "$?" in
        2)
            cd "$PJM_OUT"
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
alias rmpj='pjm_fn remove -p'
