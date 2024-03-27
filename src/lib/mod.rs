mod memory;
mod utils;

pub mod types {
    pub use std::ffi::c_void;

    #[derive(Default, Debug)]
    pub struct DWORD(pub u32);

    #[derive(Clone, Copy, Debug)]
    pub struct LPBYTE(pub *mut u8);

    impl Default for LPBYTE {
        fn default() -> Self {
            Self(std::ptr::null_mut())
        }
    }
}

pub mod prelude {
    #[allow(dead_code)]
    pub const CS_PROCESS_NAME: &str = "cs2.exe";

    pub use super::memory::*;
    pub use super::types::*;
    pub use super::utils::*;
}
