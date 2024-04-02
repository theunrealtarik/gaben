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
    let mut keyboard = Keyboard::new();
    keyboard.listen();

    // timers
    let mut bunny_man = Timer::default();
    let mut keys_timer = Timer::default();

    // mind games loop
    if let Ok(process) = Memory::new(CS_PROCESS_NAME) {
        let client = process.modules.get("client.dll").unwrap().address;
        let engine = process.modules.get("engine2.dll").unwrap().address;

        loop {
            let Ok(local_player) = process.read::<usize>(client + client_dll::dwLocalPlayerPawn)
            else {
                continue;
            };

            let player_team = process.read::<u8>(local_player + client_dll::config::m_iTeamNum);
            let health = process.read::<i32>(local_player + client_dll::config::m_iHealth)?;

            let flags = process.read::<u32>(local_player + client_dll::config::m_fFlags)?;
            let force_jump = client + client_dll::dwForceJump;
            let is_grounded = flags & (1 << 0) != 0;

            if bunny_man.elapsed(Duration::from_millis(70)) {
                if is_grounded {
                    process.write::<i32>(force_jump, Modifier::Plus as i32)?;
                } else {
                    process.write::<i32>(force_jump, Modifier::Minus as i32)?;
                }
            }

            let camera_servies =
                process.read::<usize>(local_player + client_dll::config::m_pCameraServices)?;
            let is_scoped = process.read::<bool>(local_player + client_dll::config::m_bIsScoped)?;

            if !is_scoped {
                process.write::<i32>(camera_servies + client_dll::config::m_iFOV, 320)?;
            }

            thread::sleep(Duration::from_millis(16));
        }
    }

    Ok(())
}

fn get_entities(process: &Memory, client: Module) -> Result<Vec<Entity>, &str> {
    let entities = Vec::new();

    let Ok(entity_list) = process.read::<usize>(client.address + client_dll::dwEntityList) else {
        return Err("failed to get entity list");
    };

    for i in 0..64 {
        let Ok(entry) = process.read::<usize>(entity_list + 0x8 * (i >> 9) + 0x10) else {
            continue;
        };

        let Ok(controller) = process.read::<usize>(entry + 120 * (i & 0x7FFF)) else {
            continue;
        };

        let Ok(pawn_handle) = process.read::<usize>(controller + client_dll::config::m_hPlayerPawn)
        else {
            continue;
        };

        if let Ok(pawn_entry) =
            process.read::<usize>(entity_list + 0x8 * ((pawn_handle & 0x7FFF) >> 9) + 0x10)
        {
            match process.read::<usize>(pawn_entry + 120 * (pawn_handle & 0x1FFF)) {
                Ok(pawn) => {
                    let weapon_id = 0;

                    let entity = Entity::new(
                        process
                            .read::<u32>(pawn + client_dll::config::m_iPawnHealth)
                            .unwrap(),
                        process
                            .read::<bool>(pawn + client_dll::config::m_bPawnIsAlive)
                            .unwrap(),
                        process
                            .read::<bool>(
                                pawn + client_dll::config::m_entitySpottedState
                                    + client_dll::config::m_bSpotted,
                            )
                            .unwrap(),
                        process
                            .read::<u8>(pawn + client_dll::config::m_iTeamNum)
                            .unwrap(),
                        weapon_id,
                    );
                }
                Err(_) => continue,
            }
        };
    }

    Ok(entities)
}
