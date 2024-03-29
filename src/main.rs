#![allow(special_module_name)]
mod lib;
mod test;

use lib::prelude::*;

mod offsets {
    pub const LOCAL_PLAYER: isize = 0x17371A8;
    pub const HEALTH: isize = 0x334;
}

#[cfg(target_os = "windows")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(process) = Memory::new(CS_PROCESS_NAME) {
        let client = process.modules.get("client.dll").unwrap().address;
        let local_player = unsafe { client.offset(offsets::LOCAL_PLAYER) };

        let health = process.read_pointer::<i32>(local_player, Some(&[offsets::HEALTH]))?;
    }

    Ok(())
}
