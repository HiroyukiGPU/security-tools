fn main(){for (k,v) in [("TOOL_ID","skillpermission"),("TOOL_BIN","skillpermission"),("TOOL_DESC","スキル起動の作業場所制限と子プロセス制御"),("TOOL_PROFILE","sandbox")]{println!("cargo:rustc-env={k}={v}");}}
