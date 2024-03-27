pub use std::ffi::c_void;

pub type DWORD = u32;
pub type LPCVOID = *const c_void;
pub type LPVOID = *mut c_void;

#[derive(Clone, Copy, Debug)]
pub struct LPBYTE(pub *mut u8);

impl Default for LPBYTE {
    fn default() -> Self {
        Self(std::ptr::null_mut())
    }
}
