fn main(){for (k,v) in [("TOOL_ID","agentscope"),("TOOL_BIN","agentscope"),("TOOL_DESC","AIエージェント起動前の作業場所と指示ファイル境界の検証"),("TOOL_PROFILE","agent-instructions")]{println!("cargo:rustc-env={k}={v}");}}
