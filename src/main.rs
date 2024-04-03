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
    use crate::sdk::offsets;

    let mut keyboard = Keyboard::new();
    keyboard.listen();

    // timers
    let mut bunny_man = Timer::default();
    let mut keys_timer = Timer::default();

    // mind games loop
    if let Ok(process) = Memory::new(CS_PROCESS_NAME) {
        let client = process.modules.get("client.dll").unwrap();

        loop {
            let Ok(local_player) = process.read::<usize>(client.address + offsets::DW_LOCAL_PAWN)
            else {
                continue;
            };

            let Some(player) = Player::new(&process, local_player) else {
                continue;
            };

            let force_jump = client.address + offsets::buttons::jump;
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
                if *entity.spotted()
                    && keys_timer.elapsed(Duration::from_millis(500))
                    && MouseButton::LeftButton.is_pressed()
                    && rand::random::<bool>()
                    && (!player.weapon().is_throwable())
                {
                    keyboard.key_click(EmuKey::G);
                    println!("{:?}", entity.weapon());
                }
            }

            // let camera_servies =
            //     process.read::<usize>(local_player + client_dll::pawn::m_pCameraServices)?;
            thread::sleep(Duration::from_millis(16));
        }
    }

    Ok(())
}
