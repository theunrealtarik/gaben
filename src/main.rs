#![allow(special_module_name)]
mod lib;
mod sdk;
mod test;

use lib::prelude::*;
use sdk::prelude::*;

use std::time::Duration;

#[cfg(target_os = "windows")]
fn main() -> Result<(), anyhow::Error> {
    use std::thread;

    Keyboard::listen();

    // timers
    let mut bunny_man = Timer::default();

    // mind games loop
    if let Ok(process) = Memory::new(CS_PROCESS_NAME) {
        let client = process.modules.get("client.dll").unwrap().address;
        let engine = process.modules.get("engine2.dll").unwrap().address;

        let _game_state = process.read_pointer::<i32>(
            unsafe { engine.offset(engine2_dll::dwNetworkGameClient) },
            Some(&[engine2_dll::dwNetworkGameClient_signOnState]),
        )?;

        loop {
            let local_player = unsafe { client.offset(client_dll::dwLocalPlayerPawn) };
            let Ok(health) =
                process.read_pointer::<i32>(local_player, Some(&[client_dll::config::m_iHealth]))
            else {
                continue;
            };

            let force_jump = unsafe { client.offset(client_dll::dwForceJump) };
            let player_flags = process
                .read_pointer::<DWORD>(local_player, Some(&[client_dll::config::m_fFlags]))?;

            let is_alive = health > 0 && health <= 100;
            let is_grounded = player_flags & (1 << 0) != 0;

            if !is_alive {
                continue;
            }

            if bunny_man.elapsed(Duration::from_millis(70)) {
                if is_grounded {
                    process.write::<i32>(force_jump, PlayerJump::Plus as i32)?;
                } else {
                    process.write::<i32>(force_jump, PlayerJump::Minus as i32)?;
                }
            }

            thread::sleep(Duration::from_millis(16));
        }
    }

    Ok(())
}
