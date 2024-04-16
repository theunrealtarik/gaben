use std::sync::Mutex;
use std::time::Duration;

use macros::ContinuousPunishment;
use sdk::prelude::*;

#[derive(ContinuousPunishment, Default)]
pub struct SlipperyWeapons {
    timer: Mutex<Timer>,
    schedule: PunishmentSchedule,
    name: String,
}

impl Punishment for SlipperyWeapons {
    fn schedule(&self) -> &PunishmentSchedule {
        &self.schedule
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn action(&self, _: &Memory, player: &Option<Player>, entities: &Option<Vec<Entity>>) {
        if let (Some(entities), Some(player)) = (entities, player) {
            let mut timer = self.timer.lock().unwrap();
            let spotted_entities = entities
                .into_iter()
                .filter(|entity| *entity.spotted())
                .collect::<Vec<_>>()
                .len();

            if spotted_entities > 0
                && timer.elapsed(Duration::from_millis(500))
                && MouseButton::LeftButton.is_pressed()
                && rand::random::<bool>()
                && !player.weapon().is_throwable()
                && !player.weapon().is_knife()
            {
                Keyboard::stroke(Key::GKey);
            }
        }
    }
}
