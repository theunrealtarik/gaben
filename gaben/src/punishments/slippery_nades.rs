use macros::*;
use sdk::prelude::*;

#[derive(Hash, PartialEq, Eq, Default)]
enum MouseTimer {
    #[default]
    Press,
    Release,
}

#[derive(ContinuousPunishment, Default)]
pub struct SlipperyNades {
    name: String,
    schedule: PunishmentSchedule,
}

impl Punishment for SlipperyNades {
    fn schedule(&self) -> &PunishmentSchedule {
        &self.schedule
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn action(&self, _: &Memory, player: &Option<Player>, _: &Option<Vec<Entity>>) {
        if let Some(player) = player {
            if *player.is_alive()
                && player.weapon().is_throwable()
                && !(player.weapon().is_molotov() || player.weapon().is_incendiary())
                && (MouseButton::LeftButton.is_pressed() || MouseButton::RightButton.is_pressed())
            {
                Keyboard::stroke(Key::GKey);
            }
        }
    }
}
