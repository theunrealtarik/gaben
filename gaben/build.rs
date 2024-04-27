use std::path::PathBuf;
use winres::*;

const ENGLISH: u16 = 0x009;

#[cfg(target_os = "windows")]
fn main() {
    let icon = PathBuf::from("../assets/defender.ico");
    let mut res = WindowsResource::new();
    res.set_icon(icon.to_str().unwrap());
    res.set_language(ENGLISH);
    res.set("ProductName", &env!("CAMOFLAGE"));
    res.compile().unwrap();
}
