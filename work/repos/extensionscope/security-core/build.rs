fn main(){for (k,v) in [("TOOL_ID","extensionscope"),("TOOL_BIN","extensionscope"),("TOOL_DESC","Chromium拡張機能の権限監査"),("TOOL_PROFILE","browser-extension")]{println!("cargo:rustc-env={k}={v}");}}
