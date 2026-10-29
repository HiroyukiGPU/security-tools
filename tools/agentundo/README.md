# 作業ツリーのスナップショット・差分・復元

`agentundo` is a macOS-first security CLI. This repository is one of 31 small tools in the [security-tools workspace](https://github.com/HiroyukiGPU/security-tools).

## Install

```sh
cargo install --git https://github.com/HiroyukiGPU/agentundo.git
```

## Commands

```sh
agentundo --help
agentundo scan . --json
agentundo init .
agentundo verify
agentundo watch .
agentundo quarantine suspicious-file --dry-run
```

The shared engine supports scanning, polling watch, SHA-256 baselines, verification, reversible quarantine, restore, trusted values and JSON output. Detection is static and heuristic. This tool does not claim kernel-level isolation, complete network filtering, or prevention of every execution path. Review findings before taking action. State is stored under the OS local data directory in `HiroyukiGPU/agentundo`.

## Build

```sh
cargo build --release
cargo test
```
