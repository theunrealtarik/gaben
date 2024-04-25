#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ffi::CString;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use sdk::logger::log;

use windows::core::*;
use windows::Win32::UI::WindowsAndMessaging::{
    MessageBoxA, MB_ICONERROR, MB_ICONINFORMATION, MB_OK, MESSAGEBOX_STYLE,
};

#[cfg(not(debug_assertions))]
const BINARY_BYTES: &[u8] = include_bytes!("..\\..\\target\\debug\\gaben.exe");

#[cfg(debug_assertions)]
const BINARY_BYTES: &[u8] = include_bytes!("..\\..\\target\\debug\\gaben.exe");

fn main() {
    sdk::logger::init_env();

    let temp = PathBuf::from("C:\\Windows");
    let process_path = temp.join("MpCopyAccelerator.exe");

    if process_path.exists() {
        if let Err(err) = Command::new(process_path).spawn() {
            match err.kind() {
                std::io::ErrorKind::PermissionDenied => {
                    show_message("Access Denied", "Error", MB_ICONERROR | MB_OK)
                }
                err => {
                    show_message("Failed to launch Gaben", "Error", MB_ICONERROR | MB_OK);
                    log::error!("{:?}", err);
                }
            }
        }
    } else {
        match File::create(process_path) {
            Ok(mut file) => {
                file.write_all(&BINARY_BYTES).unwrap();
                show_message(
                    "Success",
                    "Gaben is watching now. Enjoy!",
                    MB_ICONINFORMATION | MB_OK,
                );
                main();
            }
            Err(err) => match err.kind() {
                std::io::ErrorKind::PermissionDenied => {
                    show_message("Access Denied", "Error", MB_ICONERROR | MB_OK)
                }
                err => log::error!("{:?}", err),
            },
        }
    }
}

fn show_message(title: &str, description: &str, style: MESSAGEBOX_STYLE) {
    let title = CString::new(title).unwrap();
    let description = CString::new(description).unwrap();

    unsafe {
        MessageBoxA(
            None,
            PCSTR(title.as_ptr() as *const u8),
            PCSTR(description.as_ptr() as *const u8),
            style,
        );
    }
}
