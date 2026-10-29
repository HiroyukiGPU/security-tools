fn main(){for (k,v) in [("TOOL_ID","agentundo"),("TOOL_BIN","agentundo"),("TOOL_DESC","作業ツリーのスナップショット・差分・復元"),("TOOL_PROFILE","integrity")]{println!("cargo:rustc-env={k}={v}");}}
