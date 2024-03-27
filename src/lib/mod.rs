mod memory;
mod utils;

pub mod types {
    pub use std::ffi::c_void;
    pub type DWORD = u32;
}

pub mod prelude {
    #[allow(dead_code)]
    pub const CS_PROCESS_NAME: &str = "cs2.exe";

    pub use super::memory::*;
    pub use super::types::*;
    pub use super::utils::*;
}
