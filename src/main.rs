#![allow(special_module_name)]
mod lib;
mod sdk;
mod test;

use lib::prelude::*;
use sdk::prelude::*;

use std::thread;
use std::time::Duration;

#[cfg(target_os = "windows")]
fn main() -> Result<(), anyhow::Error> {
    use enigo::KeyboardControllable;

    let mut keyboard = Keyboard::new();
    keyboard.listen();

    // timers
    let mut bunny_man = Timer::default();
    let mut keys_timer = Timer::default();

    // mind games loop
    if let Ok(process) = Memory::new(CS_PROCESS_NAME) {
        let client = process.modules.get("client.dll").unwrap();

        loop {
            let Ok(local_player) =
                process.read::<usize>(client.address + client_dll::dwLocalPlayerPawn)
            else {
                continue;
            };

            let Some(player) = Player::new(&process, local_player) else {
                continue;
            };

            let force_jump = client.address + client_dll::dwForceJump;
            if bunny_man.elapsed(Duration::from_millis(70)) {
                if player.is_grounded() {
                    process.write::<i32>(force_jump, Modifier::Plus as i32)?;
                } else {
                    process.write::<i32>(force_jump, Modifier::Minus as i32)?;
                }
            }

            let Some(entities) = Entity::get_entities(&process, client) else {
                continue;
            };

            for entity in entities {
                if *entity.spotted() && keys_timer.elapsed(Duration::from_millis(500)) {
                    keyboard.key_click(EmuKey::G);
                }
            }

            // let camera_servies =
            //     process.read::<usize>(local_player + client_dll::pawn::m_pCameraServices)?;
            thread::sleep(Duration::from_millis(16));
        }
    }

    Ok(())
}
