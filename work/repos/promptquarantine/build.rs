fn main(){for (k,v) in [("TOOL_ID","promptquarantine"),("TOOL_BIN","promptquarantine"),("TOOL_DESC","プロンプトインジェクションの静的検査と隔離"),("TOOL_PROFILE","prompt")]{println!("cargo:rustc-env={k}={v}");}}
