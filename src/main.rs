#![allow(special_module_name)]
mod lib;
mod sdk;
mod test;

use lib::prelude::*;
use sdk::prelude::*;

use std::{thread, time::Duration};

#[cfg(target_os = "windows")]
#[cfg(target_pointer_width = "64")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(process) = Memory::new(CS_PROCESS_NAME) {
        let client = process.modules.get("client.dll").unwrap().address;
        let engine = process.modules.get("engine2.dll").unwrap().address;

        let _game_state = process.read_pointer::<i32>(
            unsafe { engine.offset(engine2_dll::dwNetworkGameClient) },
            Some(&[engine2_dll::dwNetworkGameClient_signOnState]),
        )?;

        let mut bunny_timer = Timer::default();

        loop {
            let local_player = unsafe { client.offset(client_dll::dwLocalPlayerPawn) };
            let Ok(_health) =
                process.read_pointer::<i32>(local_player, Some(&[client_dll::entity::m_iHealth]))
            else {
                continue;
            };

            let force_jump = unsafe { client.offset(client_dll::dwForceJump) };
            let player_flags = process
                .read_pointer::<DWORD>(local_player, Some(&[client_dll::entity::m_fFlags]))?;

            let is_grounded = player_flags == PlayerState::Standing as u32
                || player_flags == PlayerState::Crouching as u32;
            if is_grounded {
                if bunny_timer.every(Duration::from_millis(1)) {
                    process.write::<DWORD>(force_jump, PlayerJump::Plus as u32)?;
                    println!("teest");
                }
            } else {
                process.write::<DWORD>(force_jump, PlayerJump::Minus as u32)?;
            }
        }
    }

    Ok(())
}
