fn main(){for (k,v) in [("TOOL_ID","mcp-endpoint-lock"),("TOOL_BIN","mcplock"),("TOOL_DESC","MCP接続先設定の監視と許可リスト"),("TOOL_PROFILE","mcp-config")]{println!("cargo:rustc-env={k}={v}");}}
