use macros::ContinuousPunishment;
use sdk::prelude::*;

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

    fn action(&self, process: &Memory, player: &Option<Player>, entities: &Option<Vec<Entity>>) {
        match player {
            Some(player) => {
                if *player.is_alive() {
                    // println!("{:?}", player.cross_entity());
                }
            }
            None => {}
        }
    }
}
