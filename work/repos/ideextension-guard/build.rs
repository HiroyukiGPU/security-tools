fn main(){for (k,v) in [("TOOL_ID","ideextension-guard"),("TOOL_BIN","ideguard"),("TOOL_DESC","VS Code拡張機能ファイルの監視・信頼管理"),("TOOL_PROFILE","vscode-extension")]{println!("cargo:rustc-env={k}={v}");}}
