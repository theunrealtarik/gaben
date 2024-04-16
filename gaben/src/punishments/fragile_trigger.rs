use macros::ContinuousPunishment;
use sdk::{offsets::DW_ENTITY_LIST, prelude::*};

const TEAM_NUM: usize = offsets::C_BaseEntity::m_iTeamNum;
const PAWN_IS_ALIVE: usize = offsets::CCSPlayerController::m_bPawnIsAlive;

#[derive(ContinuousPunishment, Default)]
pub struct FragileTrigger {
    schedule: PunishmentSchedule,
    name: String,
}

impl Punishment for FragileTrigger {
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

        match player.entity_index() {
            Some(entity_id) => {
                let client = process.modules.get("client.dll").unwrap();
                let Ok(entity_list) = process.read::<usize>(client.address + DW_ENTITY_LIST) else {
                    return;
                };

                let Ok(entry) = process.read::<usize>(entity_list + 0x8 * (entity_id >> 9) + 0x10)
                else {
                    return;
                };

                let Ok(controller) = process.read::<usize>(entry + 120 * (entity_id & 0x1FF))
                else {
                    return;
                };

                let Ok(entity_team) = process.read::<u8>(controller + TEAM_NUM) else {
                    return;
                };

                if *player.team() as u8 == entity_team {
                    Mouse::click(MouseButton::LeftButton);
                }
            }
            None => return,
        }
    }
}
