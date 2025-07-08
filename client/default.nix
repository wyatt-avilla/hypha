{
  pkgs,
  system,
  esp-dev,
  ...
}:
let
  rustVersion = "1.86.0.0";
  toolchainVersion = "1.86.0.0";

  espRustSource = pkgs.stdenv.mkDerivation {
    name = "esp-rust-source";

    src = pkgs.fetchurl {
      url = "https://github.com/esp-rs/rust-build/releases/download/v${rustVersion}/rust-src-${rustVersion}.tar.xz";
      sha256 = "sha256-EPoxNiYUk6XZfU886bmLruXMWCiXEf5vJCSY/09lspo=";
    };

    buildInputs = [ ];

    unpackPhase = ''
      mkdir -p $out
      tar -xf $src -C $out --strip-components=1
    '';

    patchPhase = ''
      patchShebangs $out/install.sh
    '';

    outputs = [ "out" ];

    installPhase = ''
      $out/install.sh --destdir=$out --prefix="" --disable-ldconfig
      runHook postInstall
    '';
  };

  espRustToolchain = pkgs.stdenv.mkDerivation {
    name = "esp-rust-toolchain";

    src = pkgs.fetchurl {
      url = "https://github.com/esp-rs/rust-build/releases/download/v${toolchainVersion}/rust-${toolchainVersion}-x86_64-unknown-linux-gnu.tar.xz";
      sha256 = "sha256-CqqIgIvYfI10aXTRpS3TnyaMCpsRtdCaMnW3r+qN1V0=";
    };

    buildInputs = with pkgs; [
      espRustSource
      libgcc
      libz
      libcxx
      autoPatchelfHook
    ];

    unpackPhase = ''
      mkdir -p $out
      tar -xf $src -C $out --strip-components=1
    '';

    patchPhase = ''
      patchShebangs $out/install.sh
    '';

    installPhase = ''
      chmod +x $out/install.sh
      sh $out/install.sh --destdir=$out --prefix="" --disable-ldconfig
      cp -rf ${espRustSource}/rust-src/lib/rustlib/* $out/lib/rustlib/
      runHook postInstall
    '';
  };

  clientDevShell = pkgs.mkShell {
    name = "client";
    nativeBuildInputs = with pkgs; [
      espRustToolchain
      espflash

      esp-dev.packages.${system}.esp-idf-esp32
      pkg-config
      cmake
      ninja
      python3
      ldproxy
    ];

    buildInputs = with pkgs; [
      (rustPlatform.bindgenHook.override { inherit (pkgs.llvmPackages_20) clang; })
      openssl
      glibc_multi.dev
    ];

    shellHook = ''
      export CARGO_BUILD_TARGET="xtensa-esp32-espidf"
      export PATH="${espRustToolchain}/bin:$PATH"
      BINDGEN_EXTRA_CLANG_ARGS="$BINDGEN_EXTRA_CLANG_ARGS -include ${pkgs.glibc_multi.dev}/include/features.h"
    '';
  };
in
if system == "x86_64-linux" then { devShell = clientDevShell; } else { }
