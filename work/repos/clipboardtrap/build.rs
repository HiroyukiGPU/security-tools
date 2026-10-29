fn main(){for (k,v) in [("TOOL_ID","clipboardtrap"),("TOOL_BIN","clipboardtrap"),("TOOL_DESC","クリップボードの危険コマンド検査"),("TOOL_PROFILE","shell-command")]{println!("cargo:rustc-env={k}={v}");}}
