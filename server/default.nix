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

  nixosModules.server = with pkgs.lib; {
    options.hypha-server = {
      enable = mkEnableOption "Hypha server";

      port = mkOption {
        type = types.int;
        default = 8910;
        description = "Port to host the server on";
      };

      workers = mkOption {
        type = types.int;
        default = 1;
        description = "Number of workers to use for the server";
      };

      queryServices = mkOption {
        type = types.listOf types.str;
        default = [ ];
        description = "Service names to monitor";
        example = "[ \"polkit.service\" \"syncthing.service\" ]";
      };

      logLevel = mkOption {
        type = types.str;
        default = "INFO";
        description = "Log level, one of (INFO, WARN, ERROR, DEBUG, TRACE)";
      };
    };

    config = mkIf config.services.server.enable {
      systemd.services.server = {
        description = "Hypha server";
        after = [ "network.target" ];
        wantedBy = [ "multi-user.target" ];

        serviceConfig = {
          ExecStart = "${lib.getExe pkgs.server} --port ${toString config.services.server.port} --workers ${toString config.services.server.workers} --log-level ${toString config.services.server.logLevel} --services ${toString config.services.server.queryServices}";
          Restart = "always";
          User = "hypha-server";
          Group = "hypha-server";
        };
      };

      users.users.hypha-server = {
        isSystemUser = true;
        group = "hypha-server";
      };

      users.groups.hypha-server = { };

      networking.firewall.allowedTCPPorts = mkIf config.services.server.enable [
        config.services.server.port
      ];
    };
  };
}
