fn main(){for (k,v) in [("TOOL_ID","rulemutation"),("TOOL_BIN","rulemutation"),("TOOL_DESC","AI設定変更の検知とロールバック"),("TOOL_PROFILE","ai-config")]{println!("cargo:rustc-env={k}={v}");}}
