use std::process::{exit, Command, Stdio};

fn main() {
    let status = Command::new("touch").arg("post").status().expect("hello");
    assert!(status.success());
}