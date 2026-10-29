fn main(){for (k,v) in [("TOOL_ID","terminalgate"),("TOOL_BIN","terminalgate"),("TOOL_DESC","シェルコマンドの危険パターン検査と承認"),("TOOL_PROFILE","shell-command")]{println!("cargo:rustc-env={k}={v}");}}
