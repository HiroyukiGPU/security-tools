fn main(){for (k,v) in [("TOOL_ID","launchagent-watch"),("TOOL_BIN","launchwatch"),("TOOL_DESC","ユーザーLaunchAgentsの監視と隔離"),("TOOL_PROFILE","launch-agent")]{println!("cargo:rustc-env={k}={v}");}}
