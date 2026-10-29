# 依存パッケージ名と宣言の検査

`packagetwin` is a macOS-first security CLI. This repository is one of 31 small tools in the [security-tools workspace](https://github.com/HiroyukiGPU/security-tools).

## Install

```sh
cargo install --git https://github.com/HiroyukiGPU/packagetwin.git
```

## Commands

```sh
packagetwin --help
packagetwin scan . --json
packagetwin init .
packagetwin verify
packagetwin watch .
packagetwin quarantine suspicious-file --dry-run
```

The shared engine supports scanning, polling watch, SHA-256 baselines, verification, reversible quarantine, restore, trusted values and JSON output. Detection is static and heuristic. This tool does not claim kernel-level isolation, complete network filtering, or prevention of every execution path. Review findings before taking action. State is stored under the OS local data directory in `HiroyukiGPU/packagetwin`.

## Build

```sh
cargo build --release
cargo test
```
