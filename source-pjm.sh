#!/bin/sh
# https://stackoverflow.com/questions/3515208/can-colorized-output-be-captured-via-shell-redirect
function pjm_fn() {
    PJM_ARGS="$@"
    #    PJM_OUT=`script --flush --quiet --return /tmp/pjm1-out.txt --command "pjm1 ${PJM_ARGS}"`
    PJM_OUT=`pjm1 ${PJM_ARGS}`
    case "$?" in
        2)
            cd "${PJM_OUT::-1}"
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
alias rmpj='pjm_fn remove --project'
alias shpj='pjm_fn show'
