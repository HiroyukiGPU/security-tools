fn main(){for (k,v) in [("TOOL_ID","repoarrival-guard"),("TOOL_BIN","repoarrival"),("TOOL_DESC","リポジトリ展開後の実行ファイルとAI指示の検査"),("TOOL_PROFILE","repository")]{println!("cargo:rustc-env={k}={v}");}}
