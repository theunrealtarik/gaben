use macros::*;
use sdk::prelude::*;

#[derive(ContinuousPunishment, Default)]
pub struct HeavyKnife {
    name: String,
    schedule: PunishmentSchedule,
}

impl Punishment for HeavyKnife {
    fn schedule(&self) -> &PunishmentSchedule {
        &self.schedule
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn action(&self, _: &Memory, player: &Option<Player>, _: &Option<Vec<Entity>>) {
        if let Some(player) = player {
            if *player.is_alive() && player.weapon().is_knife() {
                Keyboard::stroke(Key::LShiftKey);
            }
        }
    }
}
