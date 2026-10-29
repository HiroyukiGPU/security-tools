fn main(){for (k,v) in [("TOOL_ID","agentsandbox"),("TOOL_BIN","agentsandbox"),("TOOL_DESC","隔離ランタイムの利用可能性確認と制限付き起動"),("TOOL_PROFILE","sandbox")]{println!("cargo:rustc-env={k}={v}");}}
