# Fish Fight Launcher

[![Build Status](https://img.shields.io/github/workflow/status/fishfight/Launcher/Continuous%20Integration?logo=github&labelColor=1e1c24&color=8bcfcf)](https://github.com/fishfight/Launcher/actions) [![License](https://img.shields.io/badge/License-MIT%20or%20Apache%202-green.svg?label=license&labelColor=1e1c24&color=34925e)](#license) [![Discord](https://img.shields.io/badge/chat-on%20discord-green.svg?logo=discord&logoColor=fff&labelColor=1e1c24&color=8d5b3f)](https://discord.gg/4smxjcheE5)

A cross-platform launcher for [Fish Fight](https://github.com/fishfight/FishFight).

## Features

- [x] Download and launch via CLI/GUI
- [ ] Auto updates
- [ ] Mod management

## Download

See [available releases](https://github.com/fishfight/Launcher/releases).

## Build from source

1. Clone the repository.
2. Run `cargo build --release`

## Usage

### CLI

```
fish-launcher-cli [OPTIONS] <SUBCOMMAND>
```

```
OPTIONS:
    -v, --verbose    Increase logging verbosity
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    info       List available releases
    install    Download and install the game
    launch     Launch the game
    help       Print this message or the help of the given subcommand(s)
```

#### Examples

List available releases:

```sh
fish-launcher-cli
```

Install the latest version:

```sh
fish-launcher-cli install
```

Launch the game:

```sh
fish-launcher-cli launch
```

### GUI

> Work in progress.

#### License

<sup>
All code is licensed under <a href="LICENSE-MIT">The MIT License</a> or <a href="LICENSE-APACHE">Apache 2.0 License</a>.
</sup>
