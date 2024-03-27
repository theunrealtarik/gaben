#![allow(special_module_name)]
mod lib;
mod test;

use lib::prelude::*;

#[cfg(target_os = "windows")]
fn main() {
    if let Ok(mem) = Memory::new("program.exe") {
        println!("MODULE BASE ADDRESS {:x}", mem.base_module.address);
        println!("KERNEL32.dll {:#?}", mem.modules.get("kernel32.dll"));
    }
}
