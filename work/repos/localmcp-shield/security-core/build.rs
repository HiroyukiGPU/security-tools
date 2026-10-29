fn main(){for (k,v) in [("TOOL_ID","localmcp-shield"),("TOOL_BIN","localmcp"),("TOOL_DESC","ローカルMCPサーバー設定と待受アドレスの監査"),("TOOL_PROFILE","local-service")]{println!("cargo:rustc-env={k}={v}");}}
