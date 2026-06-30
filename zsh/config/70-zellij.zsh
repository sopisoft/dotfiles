# Auto-start zellij only for the outermost interactive shell.
if [[ -o interactive ]] && [[ -z ${ZELLIJ:-} ]] && [[ ${TERM:-} != dumb ]] && command -v zellij >/dev/null 2>&1; then
    exec zellij
fi
