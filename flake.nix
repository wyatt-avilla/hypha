{
  description = "service health LED indicators with embedded Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    esp-dev.url = "github:mirrexagon/nixpkgs-esp-dev";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      esp-dev,
      rust-overlay,
    }:
    let
      nixosModules = {
        server = import ./server/module.nix { flake = self; };
      };
    in
    {
      inherit nixosModules;
    }
    // flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        serverOutputs = import ./server {
          inherit
            self
            nixpkgs
            flake-utils
            rust-overlay
            system
            pkgs
            ;
        };

        clientOutputs = import ./client {
          inherit
            self
            pkgs
            system
            esp-dev
            ;
        };
      in
      {
        devShells = clientOutputs.devShells // serverOutputs.devShells;
        packages = clientOutputs.packages // serverOutputs.packages;
        inherit (clientOutputs) apps;
      }
    );
}
