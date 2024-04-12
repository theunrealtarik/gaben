#![allow(dead_code)]
pub use std::ffi::c_void;
pub type DWORD = u32;
pub type LPCVOID = *const c_void;
pub type LPVOID = *mut c_void;
pub type LPBYTE = *mut u8;
