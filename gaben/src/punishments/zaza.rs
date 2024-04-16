use std::sync::Mutex;

use macros::*;
use sdk::prelude::*;

const ZAZA_FOV: u32 = 200;
const DEFAULT_FOV: u32 = 68;
const CAMERA_SERVICES: usize = offsets::C_BasePlayerPawn::m_pCameraServices;
const I_FOV: usize = offsets::CCSPlayerBase_CameraServices::m_iFOV;

#[derive(PeriodicPunishment, Default)]
pub struct Zaza {
    schedule: PunishmentSchedule,
    name: String,
    default_fov: Mutex<Option<u32>>,
}

impl Punishment for Zaza {
    fn schedule(&self) -> &PunishmentSchedule {
        &self.schedule
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn action(&self, process: &Memory, player: &Option<Player>, _: &Option<Vec<Entity>>) {
        let Some(player) = player else {
            return;
        };

        let Ok(camera_services) = process.read::<usize>(player.base_address() + CAMERA_SERVICES)
        else {
            return;
        };

        let mut default_fov = self.default_fov.lock().unwrap();
        if default_fov.is_none() {
            *default_fov = process.read::<u32>(camera_services + I_FOV).ok();
        }

        if !*player.is_scopped() {
            process
                .write(camera_services + I_FOV, ZAZA_FOV)
                .unwrap_or_else(|_| 0);
        }
    }

    fn withdraw(&self, process: &Memory, player: &Option<Player>, _: &Option<Vec<Entity>>) {
        if let Some(player) = player {
            if let Ok(camera_services) =
                process.read::<usize>(player.base_address() + CAMERA_SERVICES)
            {
                let bytes = process.write(camera_services + I_FOV, DEFAULT_FOV);
                dbg!(&bytes);
            }
        }
    }
}
