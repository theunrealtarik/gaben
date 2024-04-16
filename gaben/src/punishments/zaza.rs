use macros::*;
use sdk::prelude::*;

const DEFAULT_FOV: u32 = 68;
const ZAZA_FOV: u32 = 250;
const CAMERA_SERVICES: usize = offsets::C_BasePlayerPawn::m_pCameraServices;
const I_FOV: usize = offsets::CCSPlayerBase_CameraServices::m_iFOV;

#[derive(PeriodicPunishment, Default)]
pub struct Zaza {
    schedule: PunishmentSchedule,
    name: String,
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

        let Ok(curr_fov) = process.read::<u32>(camera_services + I_FOV) else {
            return;
        };

        if !*player.is_scopped() && curr_fov != ZAZA_FOV {
            process
                .write(camera_services + I_FOV, ZAZA_FOV)
                .unwrap_or_else(|_| 0);
        }
    }

    fn withdraw(&self, process: &Memory, player: &Option<Player>, _: &Option<Vec<Entity>>) {
        let Some(player) = player else {
            return;
        };

        let Ok(camera_services) = process.read::<usize>(player.base_address() + CAMERA_SERVICES)
        else {
            return;
        };

        let Ok(curr_fov) = process.read::<u32>(camera_services + I_FOV) else {
            return;
        };

        if !*player.is_scopped() && curr_fov != DEFAULT_FOV {
            process
                .write(camera_services + I_FOV, DEFAULT_FOV)
                .unwrap_or_else(|_| 0);
        }
    }
}
