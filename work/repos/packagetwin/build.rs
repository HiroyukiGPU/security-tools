fn main(){for (k,v) in [("TOOL_ID","packagetwin"),("TOOL_BIN","packagetwin"),("TOOL_DESC","依存パッケージ名と宣言の検査"),("TOOL_PROFILE","package-name")]{println!("cargo:rustc-env={k}={v}");}}
