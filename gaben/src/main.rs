mod punishments;

use punishments::*;
use sdk::prelude::*;

#[cfg(target_os = "windows")]
fn main() -> Result<(), anyhow::Error> {
    Keyboard::listen();

    if let Ok(process) = Memory::new(CS_PROCESS_NAME) {
        let client = process.modules.get("client.dll").unwrap();

        loop {
            let Ok(local_player) = process.read::<usize>(client.address + offsets::DW_LOCAL_PAWN)
            else {
                continue;
            };

            let (Some(player), Some(entities)) = (
                Player::new(&process, local_player),
                Entity::get_entities(&process, client),
            ) else {
                continue;
            };
        }
    }

    Ok(())
}
