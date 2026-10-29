fn main(){for (k,v) in [("TOOL_ID","incident-capsule"),("TOOL_BIN","incidentcapsule"),("TOOL_DESC","プロセス情報と証拠ハッシュの記録"),("TOOL_PROFILE","incident")]{println!("cargo:rustc-env={k}={v}");}}
