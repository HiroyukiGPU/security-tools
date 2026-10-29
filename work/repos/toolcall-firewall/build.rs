fn main(){for (k,v) in [("TOOL_ID","toolcall-firewall"),("TOOL_BIN","toolcallfw"),("TOOL_DESC","MCPツール呼び出しのポリシー検査と実行制御"),("TOOL_PROFILE","tool-call")]{println!("cargo:rustc-env={k}={v}");}}
