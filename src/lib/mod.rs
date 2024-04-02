pub mod keyboard;
pub mod memory;
pub mod time;
pub mod types;
pub mod utils;

#[allow(dead_code, unused_imports)]
pub mod prelude {
    pub const CS_PROCESS_NAME: &str = "cs2.exe";

    pub use super::keyboard::*;
    pub use super::memory::*;
    pub use super::time::*;
    pub use super::types::*;
    pub use super::utils::*;
}
