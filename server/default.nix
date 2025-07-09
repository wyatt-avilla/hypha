{ pkgs, ... }:
let
  nativeRustToolchain = with pkgs; [
    (rust-bin.stable.latest.default.override {
      extensions = [
        "clippy"
        "rust-src"
      ];
    })
  ];
in
{
  devShells.server = pkgs.mkShell {
    name = "server";
    nativeBuildInputs = nativeRustToolchain ++ [ pkgs.rust-analyzer ];
  };

  packages.server = pkgs.rustPlatform.buildRustPackage {
    name = "server";
    pname = "hypha-server";
    cargoLock = {
      lockFile = ../Cargo.lock;
    };
    buildAndTestSubdir = "server";
    src = ../.;

    nativeBuildInputs = nativeRustToolchain;
  };
}
