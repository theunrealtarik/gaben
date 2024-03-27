#![allow(special_module_name)]
mod lib;
mod test;

use lib::prelude::*;

#[cfg(target_os = "windows")]
fn main() {
    if let Ok(process) = Memory::new(CS_PROCESS_NAME) {
        let client = process.modules.get("client.dll").unwrap().address;

        let addr = unsafe { client.0.offset(0x17371A8) };
        println!("{:?}", addr);
    }
}
