#![allow(special_module_name)]
mod lib;
mod test;

use lib::prelude::*;

#[cfg(target_os = "windows")]
fn main() {
    if let Ok(process) = Memory::new("program.exe") {
        println!("PROCESS {:#?}", process);
        println!(
            "BASE MODULE ADDRESS 0x????{:x}",
            process.base_module.address
        );
    }
}
