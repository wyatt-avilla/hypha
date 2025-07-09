{
  pkgs,
  system,
  esp-dev,
  ...
}:
let
  toolchainVersion = "1.88.0.0";

  espRustSource = pkgs.stdenv.mkDerivation {
    name = "esp-rust-source";

    src = pkgs.fetchurl {
      url = "https://github.com/esp-rs/rust-build/releases/download/v${toolchainVersion}/rust-src-${toolchainVersion}.tar.xz";
      sha256 = "sha256-m35u//UHO7uFtQ5mn/mVhNuJ1PCsuljgkD3Rmv3uuaE=";
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

  systemMap = {
    x86_64-linux = {
      systemSlug = "x86_64-unknown-linux-gnu";
      hash = "sha256-dFNJFHSl9yiyRIFlHUPLzq+S9438q+fLiCxr8h/uBQU=";
    };
    aarch64-linux = {
      systemSlug = "aarch64-unknown-linux-gnu";
      hash = "sha256-gXs4aujsORQM7pH8uVddhtoQq0Qq0avsJpUr6BISxV4=";
    };
    x86_64-darwin = {
      systemSlug = "x86_64-apple-darwin";
      hash = "sha256-Y3X1Th7GUfxK87MeXSz4vkfNaam5Y2msXk2IxOD05Bg=";
    };
    aarch64-darwin = {
      systemSlug = "aarch64-apple-darwin";
      hash = "sha256-kuxLe/Y4HRNY1hwTcywMfEeIfhh0suJKnMe9ArFn2zo=";
    };
  };

  espRustToolchain = pkgs.stdenv.mkDerivation {
    name = "esp-rust-toolchain";

    src = pkgs.fetchurl {
      url = "https://github.com/esp-rs/rust-build/releases/download/v${toolchainVersion}/rust-${toolchainVersion}-${systemMap.${system}.systemSlug}.tar.xz";
      sha256 = systemMap.${system}.hash;
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
in
{
  devShells.client = pkgs.mkShell {
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
}
