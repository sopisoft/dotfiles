# dotfiles

Linux 向けの設定ファイルと、設定を symlink 配置する Rust 製 `dotfiles`
CLI です。パッケージ管理は用途ごとに分離しています。

| 責務 | 管理先 |
| --- | --- |
| 一般的な CLI、エディタ、ターミナル、ビルドツール、ユーザー空間アプリ | Homebrew (`brew/Brewfile`) |
| Node.js、Rust と clippy/rustfmt/rust-analyzer | mise (`mise.toml`) |
| Python、Conda、C/C++、ROS などのプロジェクト依存 | 各プロジェクトの `pixi.toml` |
| 上記3つで安全・簡潔に管理できない個別パッケージ | Nix（例外的な fallback） |
| kernel、driver、systemd、udev、PAM、desktop、audio、IME | OS 標準パッケージマネージャー |
| 設定ファイルの配置、symlink、udev rule | `dotfiles` CLI |

## 構成

```text
brew/Brewfile       Homebrew の汎用 CLI と基盤 CLI
config/             Alacritty、direnv、Neovim、Zellij、Starship など
mise.toml           このリポジトリで使う runtime/toolchain
scripts/            Linux bootstrap
src/                dotfiles CLI
udev/               OS にインストールする udev rule
zsh/                zsh の設定
```

管理優先順位は Homebrew、mise、pixi、Nix、OS 標準パッケージマネージャーです。
Homebrew は汎用 user-space、mise は切り替えたい runtime/tool、pixi は project
environment を担当します。Nix は上記で安全・簡潔に管理できないものだけに限定し、
OS 標準パッケージマネージャーは OS 統合に必要なものだけに限定します。

systemd service、kernel、driver、udev rule、PipeWire や fcitx5 の system integration
は NixOS 化せず、各ディストリビューションの OS 標準の仕組みに任せます。

pixi のリポジトリ全体用 environment は現在ありません。プロジェクト固有の
Python、Conda、ネイティブ依存が必要になったプロジェクトに `pixi.toml` を
置き、そこで `pixi install` を実行します。`.pixi/config.toml` は共有 cache
設定だけです。

## Linux bootstrap

Homebrew、mise、pixi が導入済みでない Linux では次を実行します。

```bash
bash scripts/bootstrap-linux.sh
```

スクリプトは次の順で処理し、既存の導入済みツールは再利用します。

1. Homebrew bootstrap に必要な既存コマンドを確認
2. Homebrew 本体を公式 installer で導入
3. `brew bundle --file=brew/Brewfile`
4. `mise install`
5. `pixi.toml` が存在する場合だけ `pixi install`
6. Rust 製 CLI を `~/.local/bin` に install
7. `dotfiles deploy`

OS パッケージマネージャーは自動実行しません。不足する bootstrap 依存はエラーとして
表示し、Ubuntu なら apt、Arch なら pacman などで個別に導入します。

Ubuntu と Arch で共通なのは Homebrew、mise、pixi、dotfiles と設定ファイルです。
差分は kernel、driver、systemd、desktop、audio、IME、udev などの OS 層だけです。

## CLI

```bash
dotfiles deploy
dotfiles install
dotfiles switch
dotfiles update
dotfiles healthcheck
dotfiles cleanup
dotfiles update-neovim-plugins
dotfiles sync-nvim-pack
dotfiles remote status
dotfiles backups list
dotfiles rollback [generation]
dotfiles udev apply
dotfiles udev status
```

`dotfiles` は設定を配置します。Zellij は対話シェルで自動起動します。既に Zellij
内にいるシェルでは `ZELLIJ` 環境変数を検出して再起動しません。

Starship と direnv は通常の設定ファイルとして配置します。direnv hook は
`direnv` が存在する場合だけ有効になります。

## 移行表

| 旧 Nix 管理項目 | 移行先 | 判断 |
| --- | --- | --- |
| `alacritty`、`fd`、`file`、`fzf`、`git`、`gnupg`、`jq`、`neovim` | `brew/Brewfile` | 一般的な user-space CLI/application |
| `cmake`、`clang`、`gnumake`、`lld`、`pkg-config` | `brew/Brewfile` | 汎用 build toolchain |
| `cargo`、`rustc`、`clippy`、`rustfmt`、`rust-analyzer`、`nodejs` | `mise.toml` | versioned runtime/toolchain |
| `pixi` | `brew/Brewfile` | pixi 自体は基盤 CLI |
| `python3` | 各 project の `pixi.toml` | project environment に閉じ込める |
| `dbus`、`dconf`、`pulseaudio`、`pavucontrol`、`shadow` | OS | session/audio/system integration |
| `fcitx5`、Mozc、Hazkey、font、theme | OS | IME・font・desktop integration |
| `thunar`、XFCE、`xdg-user-dirs`、Xorg、`xauth` | OS | desktop/display integration |
| `podman`、`x11vnc`、`tmux`、`zsh` | `brew/Brewfile` | user-space application/tool |
| Home Manager の `home.file` / `xdg.configFile` | `src/app/deploy.rs` | symlink 配置だけを担当 |
| NixOS、WSL、nix-direnv、Nix shell hook | 削除 | distro-specific Nix integration |

## 削除したもの

`home/`、`nixos/`、`pkgs/`、Home Manager、NixOS/WSL configuration、nix-direnv、
Nix の shell hook、Nix store cleanup、Nix bootstrap、flake は削除しています。
`rust-toolchain.toml` は toolchain の責務を `mise.toml` に移したため削除しています。
Nix は通常の構成に含めず、最後の fallback として必要な場合だけ個別に導入します。

## バックアップ

既存の通常ファイルやディレクトリと衝突する場合、配置前に
`~/.local/state/dotfiles/backups/generations/` へ退避します。symlink は managed
target として置き換えます。保持数の既定値は 10 で、`DOTFILES_BACKUP_LIMIT` で
変更できます。

## Arch などへ移植する箇所

変更が必要なのは `scripts/bootstrap-linux.sh` の OS bootstrap 部分と、OS 側で
導入する desktop、audio、IME、font、driver、udev の package 名です。
`brew/Brewfile`、`mise.toml`、設定ファイル、Rust CLI の deployment 流れは
そのまま利用できます。
