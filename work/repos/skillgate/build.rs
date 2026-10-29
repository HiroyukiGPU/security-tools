fn main(){for (k,v) in [("TOOL_ID","skillgate"),("TOOL_BIN","skillgate"),("TOOL_DESC","AIスキルの権限・スクリプト検査と承認管理"),("TOOL_PROFILE","skill")]{println!("cargo:rustc-env={k}={v}");}}
