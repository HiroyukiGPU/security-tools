fn main(){for (k,v) in [("TOOL_ID","githook-guard"),("TOOL_BIN","githookguard"),("TOOL_DESC","Git Hooksとcore.hooksPathの監査・隔離"),("TOOL_PROFILE","git-hook")]{println!("cargo:rustc-env={k}={v}");}}
