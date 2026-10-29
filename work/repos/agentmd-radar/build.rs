fn main(){for (k,v) in [("TOOL_ID","agentmd-radar"),("TOOL_BIN","agentmd"),("TOOL_DESC","AI向け指示ファイルの探索、ルール検査、隔離と復元"),("TOOL_PROFILE","agent-instructions")]{println!("cargo:rustc-env={k}={v}");}}
