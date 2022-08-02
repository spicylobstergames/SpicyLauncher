# Spicy Launcher

[![Build Status](https://img.shields.io/github/workflow/status/spicylobstergames/SpicyLauncher/Continuous%20Integration?logo=github&labelColor=1e1c24&color=8bcfcf)](https://github.com/spicylobstergames/SpicyLauncher/actions) [![License](https://img.shields.io/badge/License-MIT%20or%20Apache%202-green.svg?label=license&labelColor=1e1c24&color=34925e)](#license) [![Discord](https://img.shields.io/badge/chat-on%20discord-green.svg?logo=discord&logoColor=fff&labelColor=1e1c24&color=8d5b3f)](https://discord.gg/4smxjcheE5)

A cross-platform launcher for playing [Spicy Lobster](https://github.com/spicylobstergames) games.

![gui_preview](https://user-images.githubusercontent.com/24392180/153517081-9a8b6fb6-3901-430f-abe3-712c1dd8feb4.gif)

## Supported games

- [x] [Fish Fight: Jumpy](https://github.com/fishfight/jumpy)
- [ ] [Fish Fight: Punchy](https://github.com/fishfight/punchy)

## Features

- [x] Install and launch (via GUI/CLI)
- [ ] Auto updates
- [ ] Mod management

## Download

See [available releases](https://github.com/spicylobstergames/SpicyLauncher/releases).

## Build from source

```sh
# Build CLI
$ cd cli/
$ cargo build --release
```

```sh
# Build GUI
$ cd gui/
$ yarn install --ignore-engines
$ yarn tauri build
```

## CLI

![cli_preview](https://user-images.githubusercontent.com/24392180/153515463-847a02c6-de6b-438a-a97d-03cb56d5e7d5.gif)

### Usage

```
spicy-launcher-cli [OPTIONS] <SUBCOMMAND>
```

```
OPTIONS:
    -v, --verbose    Increase logging verbosity
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    list       List available releases
    install    Download and install the game
    uninstall  Uninstall the game
    launch     Launch the game
    help       Print this message or the help of the given subcommand(s)
```

### Examples

List available releases:

```sh
spicy-launcher-cli
```

Install the latest version:

```sh
spicy-launcher-cli install
```

Launch the game:

```sh
spicy-launcher-cli launch
```

Uninstall:

```sh
spicy-launcher-cli uninstall
```

#### License

<sup>
All code is licensed under <a href="LICENSE-MIT">The MIT License</a> or <a href="LICENSE-APACHE">Apache 2.0 License</a>.
</sup>
