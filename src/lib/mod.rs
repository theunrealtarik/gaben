mod memory;
mod utils;

pub mod prelude {
    pub const CS_PROCESS_NAME: &str = "cs2.exe";

    pub use super::memory::*;
    pub use super::utils::*;
}
