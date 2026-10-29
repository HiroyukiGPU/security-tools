fn main(){for (k,v) in [("TOOL_ID","rulestamp"),("TOOL_BIN","rulestamp"),("TOOL_DESC","設定ファイルのSHA-256保護と履歴復元"),("TOOL_PROFILE","integrity")]{println!("cargo:rustc-env={k}={v}");}}
