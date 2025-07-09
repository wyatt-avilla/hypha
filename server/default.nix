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

  packages.server =
    let
      binName = "hypha-server";
    in
    pkgs.rustPlatform.buildRustPackage {
      name = "server";
      pname = binName;
      cargoLock = {
        lockFile = ../Cargo.lock;
      };
      buildAndTestSubdir = "server";
      src = ../.;

      nativeBuildInputs = nativeRustToolchain;
      meta.mainProgram = binName;
    };
}
