{ flake }:
{
  lib,
  config,
  pkgs,
  ...
}:
let
  serverBin = lib.getExe flake.packages.${pkgs.system}.server;
in
with lib;
{
  options.services.hypha-server = {
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

  config = mkIf config.services.hypha-server.enable {
    systemd.services.hypha-server = {
      description = "Hypha server";
      after = [ "network.target" ];
      wantedBy = [ "multi-user.target" ];

      serviceConfig = {
        ExecStart = "${serverBin} --port ${toString config.services.hypha-server.port} --workers ${toString config.services.hypha-server.workers} --log-level ${config.services.hypha-server.logLevel} --services ${lib.concatStringsSep " " config.services.hypha-server.queryServices}";
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

    networking.firewall.allowedTCPPorts = mkIf config.services.hypha-server.enable [
      config.services.hypha-server.port
    ];
  };
}
