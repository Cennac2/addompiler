# addompiler


[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://www.rust-lang.org)

A CLI tool for building Minecraft Bedrock addons — copies your `BP`/`RP` source folders into your development addon directories.

## Features

- **Build** — copies `src/BP` and `src/RP` into your configured Behavior Pack / Resource Pack destinations
- **Watch** — automatically rebuilds whenever files in your project change
- **Profiles** — define named build profiles with `before_build` / `after_build` command hooks and files/folders to ignore during copy
- **Init** — scaffold a new addon project

## Installation

### From source

```sh
git clone https://github.com/Cennac2/addompiler.git
cd addompiler
cargo install --path .
```

This installs the `addompiler` binary to `~/.cargo/bin` (or `%USERPROFILE%\.cargo\bin` on Windows), which is on your `PATH` automatically if you installed Rust via `rustup`.

### Requirements

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)

## Usage

```
Usage: addompiler [OPTIONS] <ARG_TYPE>

Arguments:
  <ARG_TYPE>  init, build, watch

Options:
  -d, --directory <DIRECTORY>  Project directory to operate on [default: ./]
      --debug                  Enable verbose debug logging
  -p, --profile <PROFILE>      Build profile to use
  -h, --help                   Print help
```

### Examples

Build the addon using the default settings:

```sh
addompiler build
```

Build using a specific profile:

```sh
addompiler build -p release
```

Watch for changes and rebuild automatically:

```sh
addompiler watch
```

Scaffold a new project:

```sh
addompiler init
```

## Project layout

`addompiler` expects your addon source to live under:

```
your-project/
├── addompiler_config      # config file
└── src/
    ├── BP/               # Behavior Pack source
    └── RP/                # Resource Pack source
```

## Configuration

Configuration lives in a JSON file in the project directory.

Example config:
```json
{
  "addon_name": "my_addon",
  "paths": {
    "bp_path": "C:/path/to/development_behavior_packs",
    "rp_path": "C:/path/to/development_resource_packs"
  },
  "profiles": {
    "release": {
      "ignored_files": [ "node_modules" ],
      "before_build": [
        { "command": "echo Starting build" },
        { "command": "npx tsc" }
      ],
      "after_build": [
        { "command": "echo Build complete" }
      ]
    }
  }
}
```

| Field | Type | Description |
|---|---|---|
| `addon_name` | `string` | Used as the prefix for the destination folder names (`{addon_name}_BP`, `{addon_name}_RP`) |
| `paths.bp_path` | `string?` | Destination directory for the Behavior Pack. If omitted, the BP copy step is skipped |
| `paths.rp_path` | `string?` | Destination directory for the Resource Pack. If omitted, the RP copy step is skipped |
| `profiles` | `object` | Named build profiles, selected with `-p <name>` |

### Profiles

Each profile can define:

- **`before_build`** — shell commands run before files are copied (e.g. bundling scripts, compiling TypeScript)
- **`after_build`** — shell commands run after files are copied (e.g. notifications, packaging)
- **`ignored_files`** — glob patterns (e.g. `*.log`, `node_modules`) for files/folders to exclude from the copy, matched against paths relative to `src/BP` and `src/RP`

If no `-p`/`--profile` is passed, the build runs with no profiles and no ignored files.

## License

This project is licensed under the [MIT License](LICENSE).


## License

<!-- e.g. MIT, Apache-2.0 — add your license here -->
