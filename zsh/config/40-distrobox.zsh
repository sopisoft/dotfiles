_dbx_host_dotfiles_dir() {
    local candidate

    for candidate in "${DOTFILES_DIR:-}" "$HOME/dotfiles"; do
        [[ -n "$candidate" ]] || continue
        candidate="${candidate:A}"

        if [[ -d "$candidate" ]]; then
            print -r -- "$candidate"
            return 0
        fi
    done

    return 1
}

dbx_enter_login() {
    local container_name="$1"
    local dispatch_script
    local host_dotfiles
    shift

    host_dotfiles="$(_dbx_host_dotfiles_dir 2>/dev/null)" || true
    dispatch_script="$HOME/.config/zsh/lib/distrobox-dispatch-enter.sh"

    if [[ -n "$host_dotfiles" && -r "$host_dotfiles/zsh/config/lib/distrobox-dispatch-enter.sh" ]]; then
        dispatch_script="/run/host${host_dotfiles}/zsh/config/lib/distrobox-dispatch-enter.sh"
    fi

    command distrobox enter "$container_name" -- sh "$dispatch_script" "$@"
}

dbx_create() {
    command distrobox create \
        "$@" \
        --additional-flags "--device /dev/ttyUSB0 --device /dev/ttyACM0"
}

alias dbx-create='dbx_create'
unalias ros 2>/dev/null || true
ros() {
    dbx_enter_login ros-jazzy "$@"
}

# Containers often miss the alacritty terminfo entry and serial access bits.
if [[ -n ${container:-} || -n ${DISTROBOX_ENTER_PATH:-} || -f /run/.containerenv ]]; then
    if [[ ${TERM:-} == alacritty ]] && ! command infocmp alacritty >/dev/null 2>&1; then
        export TERM=xterm-256color
    fi

    export STARSHIP_DISTROBOX=1

    _distrobox_fix_serial_permissions() {
        for device in /dev/ttyUSB0 /dev/ttyACM0; do
            [[ -e "$device" ]] || continue
            chmod 777 "$device" >/dev/null 2>&1 || true
        done
    }

    precmd_functions+=(_distrobox_fix_serial_permissions)
    _distrobox_fix_serial_permissions
fi
