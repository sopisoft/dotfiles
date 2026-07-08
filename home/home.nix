{...}: {
  imports = [
    ./modules/base.nix
    ./modules/packages.nix
    ./modules/shell.nix
    ./modules/desktop.nix
    ./modules/browser.nix
    ./modules/terminal.nix
    ./modules/editor.nix
  ];
}
