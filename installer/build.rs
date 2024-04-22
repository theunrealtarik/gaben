use std::process::Command;
use winres::*;

#[cfg(target_os = "windows")]
fn main() {
    // Command::new("cargo")
    //     .args(["build", "--bin", "gaben", "--release"])
    //     .spawn()
    //     .expect("failed to build bait binary");

    let assets = std::env::current_dir()
        .unwrap()
        .join("installer")
        .join("assets");
    let mut res = WindowsResource::new();
    res.set_icon(assets.join("installer.ico").to_str().unwrap());
    res.set_manifest_file(assets.join("app.manifest").to_str().unwrap());
}
