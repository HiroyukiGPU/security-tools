# Security Tools

31 macOS-first Rust CLI tools for AI agent and software supply-chain security. Each tool is independently published in its own repository. The shared engine provides file scanning, polling watch, SHA-256 baselines, verification, quarantine/restore, trusted entries, JSON output, and a restricted subprocess wrapper. Profiles add targeted static heuristics.

Some requested defenses require platform components beyond a normal CLI (kernel network filtering, isolated VMs, browser managed policies). Those limits are documented by each tool. This workspace is a usable initial implementation and does not represent full protection against every attack.

## Tools

| CLI | Repository | Focus |
|---|---|---|
| `agentmd` | [agentmd-radar](https://github.com/HiroyukiGPU/agentmd-radar) | AI向け指示ファイルの探索、ルール検査、隔離と復元 |
| `agentscope` | [agentscope](https://github.com/HiroyukiGPU/agentscope) | AIエージェント起動前の作業場所と指示ファイル境界の検証 |
| `rulestamp` | [rulestamp](https://github.com/HiroyukiGPU/rulestamp) | 設定ファイルのSHA-256保護と履歴復元 |
| `rulemutation` | [rulemutation](https://github.com/HiroyukiGPU/rulemutation) | AI設定変更の検知とロールバック |
| `promptquarantine` | [promptquarantine](https://github.com/HiroyukiGPU/promptquarantine) | プロンプトインジェクションの静的検査と隔離 |
| `repoarrival` | [repoarrival-guard](https://github.com/HiroyukiGPU/repoarrival-guard) | リポジトリ展開後の実行ファイルとAI指示の検査 |
| `skillgate` | [skillgate](https://github.com/HiroyukiGPU/skillgate) | AIスキルの権限・スクリプト検査と承認管理 |
| `mcplock` | [mcp-endpoint-lock](https://github.com/HiroyukiGPU/mcp-endpoint-lock) | MCP接続先設定の監視と許可リスト |
| `toolcallfw` | [toolcall-firewall](https://github.com/HiroyukiGPU/toolcall-firewall) | MCPツール呼び出しのポリシー検査と実行制御 |
| `localmcp` | [localmcp-shield](https://github.com/HiroyukiGPU/localmcp-shield) | ローカルMCPサーバー設定と待受アドレスの監査 |
| `skillpermission` | [skillpermission](https://github.com/HiroyukiGPU/skillpermission) | スキル起動の作業場所制限と子プロセス制御 |
| `agentmemory` | [agentmemory-lock](https://github.com/HiroyukiGPU/agentmemory-lock) | AIメモリファイルの変更履歴と復元 |
| `agentsandbox` | [agentsandbox](https://github.com/HiroyukiGPU/agentsandbox) | 隔離ランタイムの利用可能性確認と制限付き起動 |
| `secretcurtain` | [secretcurtain](https://github.com/HiroyukiGPU/secretcurtain) | 秘密らしい環境変数を除いた子プロセス起動 |
| `agentundo` | [agentundo](https://github.com/HiroyukiGPU/agentundo) | 作業ツリーのスナップショット・差分・復元 |
| `agentegress` | [agentegress](https://github.com/HiroyukiGPU/agentegress) | 許可ドメイン設定の管理とプロキシ環境の起動 |
| `clipboardtrap` | [clipboardtrap](https://github.com/HiroyukiGPU/clipboardtrap) | クリップボードの危険コマンド検査 |
| `terminalgate` | [terminalgate](https://github.com/HiroyukiGPU/terminalgate) | シェルコマンドの危険パターン検査と承認 |
| `installerorigin` | [installerorigin](https://github.com/HiroyukiGPU/installerorigin) | macOSアプリの署名・公証・Gatekeeper検証 |
| `launchwatch` | [launchagent-watch](https://github.com/HiroyukiGPU/launchagent-watch) | ユーザーLaunchAgentsの監視と隔離 |
| `extensionscope` | [extensionscope](https://github.com/HiroyukiGPU/extensionscope) | Chromium拡張機能の権限監査 |
| `postinstallgate` | [postinstall-gate](https://github.com/HiroyukiGPU/postinstall-gate) | npm依存関係スクリプトの検査と安全なインストール |
| `pythonboot` | [pythonboot-sentry](https://github.com/HiroyukiGPU/pythonboot-sentry) | Python起動時の.pth/sitecustomize検査 |
| `packagetwin` | [packagetwin](https://github.com/HiroyukiGPU/packagetwin) | 依存パッケージ名と宣言の検査 |
| `lockfreeze` | [lockfile-freeze](https://github.com/HiroyukiGPU/lockfile-freeze) | 依存ロックファイルの承認済みハッシュ管理 |
| `githookguard` | [githook-guard](https://github.com/HiroyukiGPU/githook-guard) | Git Hooksとcore.hooksPathの監査・隔離 |
| `workflowgate` | [workflowgate](https://github.com/HiroyukiGPU/workflowgate) | GitHub Actions権限・外部Action参照の検査 |
| `dockerpriv` | [dockerprivilege-guard](https://github.com/HiroyukiGPU/dockerprivilege-guard) | Docker Composeの特権・危険マウント監査 |
| `ideguard` | [ideextension-guard](https://github.com/HiroyukiGPU/ideextension-guard) | VS Code拡張機能ファイルの監視・信頼管理 |
| `shellconfiglock` | [shellconfig-lock](https://github.com/HiroyukiGPU/shellconfig-lock) | zsh設定ファイルの履歴監視と復元 |
| `incidentcapsule` | [incident-capsule](https://github.com/HiroyukiGPU/incident-capsule) | プロセス情報と証拠ハッシュの記録 |
