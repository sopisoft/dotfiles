if [[ -r "$HOME/.nix-profile/etc/profile.d/hm-session-vars.sh" ]]; then
    # Home Manager exports session variables here for login shells.
    source "$HOME/.nix-profile/etc/profile.d/hm-session-vars.sh"
fi
