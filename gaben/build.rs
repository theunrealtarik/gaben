use std::path::PathBuf;
use winres::*;

#[cfg(target_os = "windows")]
fn main() {
    let icon = PathBuf::from("../assets/defender.ico");
    let mut res = WindowsResource::new();
    res.set_icon(icon.to_str().unwrap());
    res.compile().unwrap();
}
