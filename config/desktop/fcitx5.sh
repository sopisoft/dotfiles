export GTK_IM_MODULE=fcitx
export QT_IM_MODULE=fcitx
export SDL_IM_MODULE=fcitx
export XMODIFIERS=@im=fcitx
export GLFW_IM_MODULE=fcitx
export FCITX_ADDON_DIRS="$HOME/.nix-profile/lib/fcitx5:$HOME/.nix-profile/lib/x86_64-linux-gnu/fcitx5"
export FCITX_DATA_DIRS="$HOME/.nix-profile/share/fcitx5:${FCITX_DATA_DIRS:-}"
