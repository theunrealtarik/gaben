use winres::*;

#[cfg(target_os = "windows")]
fn main() {
    let res = WindowsResource::new();
    res.compile().unwrap();
}
