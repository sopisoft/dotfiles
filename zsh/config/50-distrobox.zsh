ros() {
    command xtask enter -- "$@"
}

if [[ -n ${container:-} || -n ${DISTROBOX_ENTER_PATH:-} || -f /run/.containerenv ]]; then
    if [[ ${TERM:-} == alacritty ]] && ! command infocmp alacritty >/dev/null 2>&1; then
        export TERM=xterm-256color
    fi

    export STARSHIP_DISTROBOX=1
fi
