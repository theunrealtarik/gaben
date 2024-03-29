mod memory;
mod time;
mod types;
mod utils;

pub mod prelude {
    #[allow(dead_code)]
    pub const CS_PROCESS_NAME: &str = "cs2.exe";

    pub use super::memory::*;
    pub use super::time::*;
    pub use super::types::*;
    pub use super::types::*;
    pub use super::utils::*;
}
