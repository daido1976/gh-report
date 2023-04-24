use std::process::{Command, Output};

// See. https://github.com/daido1976/gh-default-branch/issues/7
// $ gh auth token
pub fn fetch_token() -> String {
    let output = gh(&["auth", "token"]);
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn gh(args: &[&str]) -> Output {
    Command::new("gh")
        .args(args)
        .output()
        .expect("Failed to execute command")
}
