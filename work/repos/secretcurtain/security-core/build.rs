fn main(){for (k,v) in [("TOOL_ID","secretcurtain"),("TOOL_BIN","secretcurtain"),("TOOL_DESC","秘密らしい環境変数を除いた子プロセス起動"),("TOOL_PROFILE","secrets")]{println!("cargo:rustc-env={k}={v}");}}
