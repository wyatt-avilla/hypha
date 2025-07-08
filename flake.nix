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
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

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

        clientOutputs = import ./client { inherit pkgs system esp-dev; };

        optionalAttrs = cond: attrs: if cond then attrs else { };

        hasEsp32 = clientOutputs ? devShell;
      in
      {
        devShells = {
          inherit (serverOutputs.devShells) server;
        } // optionalAttrs hasEsp32 { client = clientOutputs.devShell; };

        inherit (serverOutputs) packages;

        inherit (serverOutputs) nixosModules;
      }
    );
}
