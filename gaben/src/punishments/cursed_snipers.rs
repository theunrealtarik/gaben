use macros::ContinuousPunishment;
use sdk::prelude::*;

#[derive(ContinuousPunishment, Default)]
pub struct CursedSnipers {
    schedule: PunishmentSchedule,
    name: String,
}

impl Punishment for CursedSnipers {
    fn schedule(&self) -> &PunishmentSchedule {
        &self.schedule
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn action(&self, _: &Memory, player: &Option<Player>, _: &Option<Vec<Entity>>) {
        if let Some(player) = player {
            match *player.weapon() {
                Weapon::AWP | Weapon::SSG08 => {
                    if *player.is_scopped() {
                        Keyboard::stroke(Key::QKey);
                    }
                }
                _ => {}
            }
        }
    }
}
