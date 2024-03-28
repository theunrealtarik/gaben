use delegate::delegate;

pub use std::ffi::c_void;
pub type DWORD = u32;
pub type LPCVOID = *const c_void;
pub type LPVOID = *mut c_void;

#[derive(Clone, Copy, Debug)]
pub struct LPBYTE(pub *mut u8);

impl LPBYTE {
    pub unsafe fn offset(&self, count: isize) -> Self {
        LPBYTE(self.0.offset(count))
    }

    delegate! {
        to self.0 {}
    }
}

impl Default for LPBYTE {
    fn default() -> Self {
        Self(std::ptr::null_mut())
    }
}
