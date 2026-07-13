{
  autoPatchelfHook,
  fetchurl,
  fcitx5,
  glibc,
  lib,
  makeWrapper,
  qt6,
  stdenv,
}: let
  zenzaiModel = fetchurl {
    url = "https://huggingface.co/Miwa-Keita/zenz-v3.1-small-gguf/resolve/main/ggml-model-Q5_K_M.gguf";
    hash = "sha256-TekwwGvvjCY6oapAaEryBttM4bljdbO47Q6lCOCxT2w=";
  };
in
  stdenv.mkDerivation rec {
    pname = "fcitx5-hazkey";
    version = "0.2.1";

    src = fetchurl {
      url = "https://github.com/7ka-Hiira/hazkey/releases/download/${version}/fcitx5-hazkey-${version}-x86_64.tar.gz";
      hash = "sha256-/u2f0L0p8h/eK347VVRGkjWZN9dD9MMB3Fxuv6d39Vs=";
    };

    sourceRoot = ".";

    nativeBuildInputs = [
      autoPatchelfHook
      makeWrapper
    ];

    dontWrapQtApps = true;

    buildInputs = [
      fcitx5
      glibc
      stdenv.cc.cc.lib
      qt6.qtbase
    ];

    installPhase = ''
      runHook preInstall

      mkdir -p "$out"
      cp -a usr/* "$out"/

      mkdir -p "$out/lib"
      if [ -d "$out/lib/x86_64-linux-gnu/fcitx5" ]; then
        mv "$out/lib/x86_64-linux-gnu/fcitx5" "$out/lib/fcitx5"
      fi
      mkdir -p "$out/lib/x86_64-linux-gnu"
      ln -s "$out/lib/fcitx5" "$out/lib/x86_64-linux-gnu/fcitx5"
      install -Dm644 ${zenzaiModel} "$out/share/hazkey/zenzai.gguf"

      rm -f "$out/bin/hazkey-settings"
      ln -s "$out/lib/x86_64-linux-gnu/hazkey/hazkey-settings" "$out/bin/hazkey-settings"

      substituteInPlace "$out/bin/hazkey-server" \
        --replace-fail 'source "$ENV_FILE"' '. "$ENV_FILE"' \
        --replace-fail 'exec "/usr/lib/x86_64-linux-gnu/hazkey/hazkey-server" "$@"' \
          'export GGML_BACKEND_DIR="'"$out"'/lib/x86_64-linux-gnu/hazkey/libllama/backends"
           export HAZKEY_ZENZAI_MODEL="'"$out"'/share/hazkey/zenzai.gguf"
           exec "'"$out"'/lib/x86_64-linux-gnu/hazkey/hazkey-server" "$@"'

      wrapProgram "$out/bin/hazkey-settings" \
        --set-default GGML_BACKEND_DIR "$out/lib/x86_64-linux-gnu/hazkey/libllama/backends" \
        --set-default HAZKEY_ZENZAI_MODEL "$out/share/hazkey/zenzai.gguf" \
        --prefix QT_PLUGIN_PATH : "${qt6.qtbase}/lib/qt-6/plugins"

      runHook postInstall
    '';

    meta = {
      description = "Japanese input method for Fcitx 5";
      homepage = "https://github.com/7ka-Hiira/hazkey";
      license = lib.licenses.gpl3Plus;
      platforms = ["x86_64-linux"];
    };
  }
