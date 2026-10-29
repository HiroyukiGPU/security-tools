# AI設定変更の検知とロールバック

`rulemutation` is a macOS-first security CLI. This repository is one of 31 small tools in the [security-tools workspace](https://github.com/HiroyukiGPU/security-tools).

## Install

```sh
cargo install --git https://github.com/HiroyukiGPU/rulemutation.git
```

## Commands

```sh
rulemutation --help
rulemutation scan . --json
rulemutation init .
rulemutation verify
rulemutation watch .
rulemutation quarantine suspicious-file --dry-run
```

The shared engine supports scanning, polling watch, SHA-256 baselines, verification, reversible quarantine, restore, trusted values and JSON output. Detection is static and heuristic. This tool does not claim kernel-level isolation, complete network filtering, or prevention of every execution path. Review findings before taking action. State is stored under the OS local data directory in `HiroyukiGPU/rulemutation`.

## Build

```sh
cargo build --release
cargo test
```
