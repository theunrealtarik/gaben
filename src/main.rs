#![allow(special_module_name)]
mod lib;
mod test;

use lib::prelude::*;

#[cfg(target_os = "windows")]
fn main() {
    if let Ok(mem) = Memory::new("notepad.exe") {
        let kernel32 = mem.modules.get("kernel32.dll");
        println!("{:#?}", mem.modules);
        println!("{:#?}", kernel32);
    }
}
