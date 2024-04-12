use std::collections::HashMap;
use std::time::Duration;

use sdk::game::*;
use sdk::inputs::*;
use sdk::memory::*;
use sdk::punishments::*;
use sdk::time::*;

#[derive(Default)]
struct SlippyWippyWeapon {
    timer: Timer,
    schedule: PunishmentSchedule,
}

impl Punishment for SlippyWippyWeapon {
    fn schedule(&self) -> &PunishmentSchedule {
        &self.schedule
    }

    fn action(
        &mut self,
        _: &HashMap<String, Module>,
        player: Option<&Player>,
        entities: Option<&Vec<Entity>>,
    ) {
        if let (Some(entities), Some(player)) = (entities, player) {
            for entity in entities {
                if *entity.spotted()
                    && self.timer.elapsed(Duration::from_millis(500))
                    && MouseButton::LeftButton.is_pressed()
                    && rand::random::<bool>()
                    && (!player.weapon().is_throwable())
                {
                    Keyboard::stroke(Key::GKey);
                }
            }
        }
    }
}

#[derive(Default)]
struct CursedSnipers {
    schedule: PunishmentSchedule,
}

impl Punishment for CursedSnipers {
    fn schedule(&self) -> &PunishmentSchedule {
        &self.schedule
    }

    fn action(
        &mut self,
        _: &HashMap<String, Module>,
        player: Option<&Player>,
        _: Option<&Vec<Entity>>,
    ) {
        if let Some(player) = player {
            match *player.weapon() {
                Weapon::AWP | Weapon::SSG08 => {
                    if *player.scopped() {
                        Keyboard::stroke(Key::QKey);
                    }
                }
                _ => {}
            }
        }
    }
}

pub struct BunnyMan {
    schedule: PunishmentSchedule,
    timer: Timer,
}

impl Default for BunnyMan {
    fn default() -> Self {
        Self {
            schedule: PunishmentSchedule::Periodic,
            timer: Timer::default(),
        }
    }
}

impl Punishment for BunnyMan {
    fn schedule(&self) -> &PunishmentSchedule {
        &self.schedule
    }

    fn action(
        &mut self,
        modules: &HashMap<String, Module>,
        player: Option<&Player>,
        entities: Option<&Vec<Entity>>,
    ) {
        let client = modules.get("client.dll").unwrap();
        let force_jump = client.address + offsets::buttons::jump;

        let Some(player) = player else {
            return;
        };

        if self.timer.elapsed(Duration::from_millis(80)) {
            if player.is_grounded() {
                // process.write::<i32>(force_jump, Modifier::Plus as i32)?;
            } else {
                // process.write::<i32>(force_jump, Modifier::Minus as i32)?;
            }
        }
    }
}
