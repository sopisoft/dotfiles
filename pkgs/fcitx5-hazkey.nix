{
  fcitx5,
  fcitx5Mozc,
  hazkeyPackage,
  makeWrapper,
  symlinkJoin,
}:
symlinkJoin {
  name = "fcitx5-with-hazkey";
  paths = [fcitx5];
  nativeBuildInputs = [makeWrapper];
  postBuild = ''
    rm "$out/bin/fcitx5"
    makeWrapper ${fcitx5}/bin/fcitx5 "$out/bin/fcitx5" \
      --set FCITX_ADDON_DIRS "${hazkeyPackage}/lib/fcitx5:${fcitx5Mozc}/lib/fcitx5:${fcitx5}/lib/fcitx5" \
      --set FCITX_DATA_DIRS "${hazkeyPackage}/share/fcitx5:${fcitx5Mozc}/share/fcitx5:${fcitx5}/share/fcitx5"
  '';
}
