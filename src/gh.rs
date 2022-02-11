use std::process::{Command, Output};

// NOTE: The hostname must be specified to fetch the oauth token.
// See. https://github.com/cli/cli/issues/4060
// $ gh config get oauth_token -h github.com
pub fn fetch_token() -> String {
    let output = gh(&["config", "get", "oauth_token", "-h", "github.com"]);
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn gh(args: &[&str]) -> Output {
    Command::new("gh")
        .args(args)
        .output()
        .expect("Failed to execute command")
}
