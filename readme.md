# Hypha

A type-safe REST API for Systemd services and accompanying ESP32 client
firmware.

## Features

- Type-safe API contract shared between server and client
- Asynchronous, message passing-based ESP32 firmware for low power consumption
- Configurable service monitoring with CLI interface

## Use Case

This stack provides a real-time visual indicator of critical Systemd service
statuses.

Personally, I'm using it to repurpose a (previously) unused hard drive status
LED on my [homelab's case](https://www.amazon.com/dp/B0CH3JXKZF).

## Installation

### Server

#### NixOS

The server is exposed as a NixOS service. Add this repo's flake to your inputs,
and enable/configure the server like so:

<details>
    <summary>Sample Flake</summary>

```nix
{
  description = "Simple example using hypha service";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    hypha.url = "github:wyatt-avilla/hypha";
  };

  outputs =
    { nixpkgs, hypha }:
    {
      nixosConfigurations.myhost = nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";
        modules = [
          hypha.nixosModules.server
          {
            services.hypha-server = {
              enable = true;
              port = 8910;
              workers = 2;
              logLevel = "INFO";
              queryServices = [
                "polkit.service"
                "NetworkManager.service"
              ];
            };
          }
        ];
      };
    };
}
```

</details>

#### Manual

For non-nix systems, compile and add the server binary to your path, and
optionally create a Systemd service.

<details>
    <summary>Sample Systemd Unit</summary>

```txt
[Unit]
After=network.target
Description=Hypha server

[Service]
ExecStart=/usr/bin/hypha-server --port 8910 --workers 1 --log-level INFO --services polkit.service tlp.service
Group=hypha-server
Restart=always
User=hypha-server

[Install]
WantedBy=multi-user.target
```

</details>

## Server

The server is built with [Actix Web](https://actix.rs/) and responds with the
statuses of the specified Systemd services on the host machine.

### Configuration

The server can be parameterized through CLI arguments.

```txt
[wyatt@nixos:~]$ hypha-server --help

Usage: hypha-server [OPTIONS] --services <SERVICES>...

Options:
  -s, --services <SERVICES>...  Space delimited service names to monitor
  -p, --port <PORT>             Port to run the server on [default: 8910]
  -w, --workers <WORKERS>       Number of workers for the server [default: 1]
  -l, --log-level <LOG_LEVEL>   Log level, one of (INFO, WARN, ERROR, DEBUG, TRACE)
  -h, --help                    Print help
  -V, --version                 Print version
```

For example, to configure server to respond with the states of the Syncthing,
Immich, and firewall services on port 8081 with 2 workers, you'd run:

```sh
hypha-server -w 2 -p 8081 -s syncthing.service immich.service firewall.service
```

### Data Shape

The `ServiceStatuses` struct in
[`api/src/lib.rs`](https://github.com/wyatt-avilla/hypha/blob/main/api/src/lib.rs)
is serialized directly for the server's API response. Systemd service states
(i.e LOAD, ACTIVE, and SUB) are modeled with enums in
[`api/src/unit.rs`](https://github.com/wyatt-avilla/hypha/blob/main/api/src/unit.rs).

### Example Json Response

```json
{
  "map": {
    "audit.service": ["Masked", "Inactive", "Exited"],
    "polkit.service": ["Loaded", "Active", "Running"]
  }
}
```

## Client

The client periodically hits the server's API endpoint and reflects the
configured Systemd service states with an LED (assumed GPIO pin 5).

The ESP32 firmware is entirely asynchronous and built with
[Embassy](https://embassy.dev/) (std). The executor tasks communicate through
channels, so the firmware as a whole is polling-free and optimized for low power
consumption.

### LED Blink Codes

| Blink                         | Description                                            |
| ----------------------------- | ------------------------------------------------------ |
| Solid, followed by $n$ blinks | $n$ of the watched services are down                   |
| Solid                         | Parse error [^1] or http client initialization failure |
| Toggled each second           | Network error                                          |

## Repo Structure

```txt
.
├── Cargo.lock
├── Cargo.toml
├── readme.md
├── api    # Shared types for the client and server
│   ├── Cargo.toml
│   └── src
├── client # Firmware
│   ├── Cargo.toml
│   └── src
└── server # API backend
    ├── Cargo.toml
    └── src
```

[^1]:
    This almost certainly won't happen. The client and server serialize and
    deserialize the same struct from the `api` crate. However, if the API
    interface changes without deployed clients being re-reflashed, then a
    deserialization error is possible.
