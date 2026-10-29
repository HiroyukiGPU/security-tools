fn main(){for (k,v) in [("TOOL_ID","shellconfig-lock"),("TOOL_BIN","shellconfiglock"),("TOOL_DESC","zsh設定ファイルの履歴監視と復元"),("TOOL_PROFILE","shell-config")]{println!("cargo:rustc-env={k}={v}");}}
