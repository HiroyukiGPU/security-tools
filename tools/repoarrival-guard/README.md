# リポジトリ展開後の実行ファイルとAI指示の検査

`repoarrival` is a macOS-first security CLI. This repository is one of 31 small tools in the [security-tools workspace](https://github.com/HiroyukiGPU/security-tools).

## Install

```sh
cargo install --git https://github.com/HiroyukiGPU/repoarrival-guard.git
```

## Commands

```sh
repoarrival --help
repoarrival scan . --json
repoarrival init .
repoarrival verify
repoarrival watch .
repoarrival quarantine suspicious-file --dry-run
```

The shared engine supports scanning, polling watch, SHA-256 baselines, verification, reversible quarantine, restore, trusted values and JSON output. Detection is static and heuristic. This tool does not claim kernel-level isolation, complete network filtering, or prevention of every execution path. Review findings before taking action. State is stored under the OS local data directory in `HiroyukiGPU/repoarrival-guard`.

## Build

```sh
cargo build --release
cargo test
```
