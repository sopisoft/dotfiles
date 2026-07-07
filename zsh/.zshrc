# Load only the managed config files in a predictable order.
for config_file in \
    00-completion.zsh \
    10-options.zsh \
    20-aliases.zsh \
    30-direnv.zsh \
    50-distrobox.zsh \
    60-ros-jazzy.zsh \
    70-prompt.zsh \
    80-zellij.zsh
do
    if [[ -r "$HOME/.config/zsh/$config_file" ]]; then
        source "$HOME/.config/zsh/$config_file"
    fi
done

unset config_file
