# Auto-start zellij only for the outermost interactive shell.
if [[ -o interactive ]] && [[ -t 0 ]] && [[ -t 1 ]] && [[ -z ${ZELLIJ:-} ]] && [[ ${TERM:-} != dumb ]] && command -v zellij >/dev/null 2>&1; then
    exec zellij
fi
