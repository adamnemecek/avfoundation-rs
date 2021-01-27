use std::process::{
    exit,
    Command,
    Stdio,
};

fn apple_darwin_sign_file(file_path: &std::path::Path) {
    let status = Command::new("codesign")
        .arg("-f") // Force to rewrite the existing signature
        .arg("-s")
        .arg("-")
        .arg(file_path)
        .status()
        .expect("failed to execute `codesign`");
    assert!(status.success());
}

fn main() {
    println!("cargo:rustc-link-lib=framework=AudioToolbox");
    println!("cargo:rustc-link-lib=framework=AVFoundation");
    println!("cargo:rustc-link-lib=framework=CoreAudioKit");
}
