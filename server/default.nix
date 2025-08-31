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
      cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
    in
    pkgs.rustPlatform.buildRustPackage {
      pname = cargoToml.package.name;
      inherit (cargoToml.package) version;

      cargoLock = {
        lockFile = ../Cargo.lock;
      };
      buildAndTestSubdir = "server";
      src = ../.;

      nativeBuildInputs = nativeRustToolchain;
      meta.mainProgram = cargoToml.bin.name;
    };
}
