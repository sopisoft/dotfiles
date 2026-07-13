#!/usr/bin/env bash
set -euo pipefail

if [[ "$(id -u)" != "0" ]]; then
  echo "This installer must run as root inside WSL." >&2
  exit 1
fi

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
git config --global --add safe.directory "${repo_root}" >/dev/null 2>&1 || true
cat >/etc/resolv.conf <<'EOF'
nameserver 1.1.1.1
nameserver 8.8.8.8
options timeout:2 attempts:3
EOF

mapfile -t candidates < <(
  lsblk -dn -b -o NAME,SIZE,TYPE |
    awk '$3 == "disk" && $2 >= 240000000000 && $2 <= 260500000000 { print "/dev/" $1 }'
)

if [[ "${#candidates[@]}" -ne 1 ]]; then
  echo "Expected exactly one 240-260GB WSL disk candidate, found ${#candidates[@]}." >&2
  lsblk -o NAME,SIZE,TYPE,FSTYPE,LABEL,MOUNTPOINTS >&2
  exit 1
fi

target_disk="${candidates[0]}"

echo "Installing NixOS to ${target_disk}"
lsblk -o NAME,SIZE,TYPE,FSTYPE,LABEL,MOUNTPOINTS "${target_disk}"

export NIX_CONFIG=$'experimental-features = nix-command flakes\naccept-flake-config = true'
export NIX_REMOTE=local
nix_flags=(--extra-experimental-features "nix-command flakes")

if [[ -x /nix/var/nix/profiles/default/bin/nix ]]; then
  nix_bin=/nix/var/nix/profiles/default/bin/nix
elif command -v nix >/dev/null 2>&1; then
  nix_bin="$(command -v nix)"
else
  echo "nix command not found. Install multi-user Nix in the Ubuntu WSL distro first." >&2
  exit 1
fi
export PATH="$(dirname "${nix_bin}"):${PATH}"

"${nix_bin}" "${nix_flags[@]}" run github:nix-community/disko -- \
  --mode disko \
  --argstr disk "${target_disk}" \
  "${repo_root}/nixos/disko-install.nix"

nixos_install="$("${nix_bin}" "${nix_flags[@]}" build --inputs-from "${repo_root}" --no-link --print-out-paths nixpkgs#nixos-install-tools)/bin/nixos-install"
system_path="$(readlink -f /root/codex-nixos-system)"

"${nix_bin}" "${nix_flags[@]}" copy --to /mnt --no-check-sigs "${system_path}"

"${nixos_install}" \
  --root /mnt \
  --system "${system_path}" \
  --no-channel-copy \
  --no-root-passwd

mkdir -p /mnt/boot/EFI/BOOT
if [[ -f /mnt/boot/EFI/systemd/systemd-bootx64.efi ]]; then
  cp -f /mnt/boot/EFI/systemd/systemd-bootx64.efi /mnt/boot/EFI/BOOT/BOOTX64.EFI
fi

sync
echo "NixOS installation finished. Initial login: sopi / nixos"
