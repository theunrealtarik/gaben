use macros::*;
use sdk::prelude::*;

#[derive(Default, ContinuousPunishment)]
pub struct FlameGrantMeStrength {
    name: String,
    schedule: PunishmentSchedule,
}

impl Punishment for FlameGrantMeStrength {
    fn schedule(&self) -> &PunishmentSchedule {
        &self.schedule
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn action(&self, _: &Memory, player: &Option<Player>, _: &Option<Vec<Entity>>) {
        let Some(player) = player else {
            return;
        };

        if *player.is_alive() && (player.weapon().is_molotov() || player.weapon().is_incendiary()) {
            Mouse::click(MouseButton::RightButton);
        }
    }
}
