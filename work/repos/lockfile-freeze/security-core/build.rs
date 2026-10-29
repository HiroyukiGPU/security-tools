fn main(){for (k,v) in [("TOOL_ID","lockfile-freeze"),("TOOL_BIN","lockfreeze"),("TOOL_DESC","依存ロックファイルの承認済みハッシュ管理"),("TOOL_PROFILE","lockfile")]{println!("cargo:rustc-env={k}={v}");}}
