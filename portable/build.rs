use winres::*;

#[cfg(windows)]
fn main() {
    let mut res = WindowsResource::new();
    res.set_manifest_file("../assets/app.manifest");
    res.set_icon("../assets/gaben.ico");
    res.compile().unwrap();
}
