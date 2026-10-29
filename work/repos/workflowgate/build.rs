fn main(){for (k,v) in [("TOOL_ID","workflowgate"),("TOOL_BIN","workflowgate"),("TOOL_DESC","GitHub Actions権限・外部Action参照の検査"),("TOOL_PROFILE","workflow")]{println!("cargo:rustc-env={k}={v}");}}
