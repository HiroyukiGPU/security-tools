fn main(){for (k,v) in [("TOOL_ID","agentmemory-lock"),("TOOL_BIN","agentmemory"),("TOOL_DESC","AIメモリファイルの変更履歴と復元"),("TOOL_PROFILE","integrity")]{println!("cargo:rustc-env={k}={v}");}}
