use winres::*;

#[cfg(target_os = "windows")]
fn main() {
    use std::path::PathBuf;

    let icon = PathBuf::from("./assets/installer.ico");
    let manifest = PathBuf::from("./assets/app.manifest");

    let mut res = WindowsResource::new();
    res.set_icon(icon.to_str().unwrap());
    res.set_manifest_file(manifest.to_str().unwrap());
    res.compile().unwrap();
}
