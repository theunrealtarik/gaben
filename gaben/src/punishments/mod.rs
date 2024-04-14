#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;

use macros::{ContinuousPunishment, PeriodicPunishment};
use sdk::game::*;
use sdk::inputs::*;
use sdk::memory::*;
pub use sdk::punishments::*;
use sdk::time::*;

#[derive(ContinuousPunishment)]
pub struct ExamplePunishment {
    pub schedule: PunishmentSchedule,
}

impl Punishment for ExamplePunishment {
    fn schedule(&self) -> &PunishmentSchedule {
        &self.schedule
    }

    fn action(&self, _: &HashMap<String, Module>, _: Option<&Player>, _: Option<&Vec<Entity>>) {
        print!("damn");
    }
}

#[derive(ContinuousPunishment)]
pub struct SlippyWippyWeapon {
    timer: Mutex<Timer>,
    schedule: PunishmentSchedule,
}

impl Punishment for SlippyWippyWeapon {
    fn schedule(&self) -> &PunishmentSchedule {
        &self.schedule
    }

    fn action(
        &self,
        _: &HashMap<String, Module>,
        player: Option<&Player>,
        entities: Option<&Vec<Entity>>,
    ) {
        if let (Some(entities), Some(player)) = (entities, player) {
            let mut timer = self.timer.lock().unwrap();
            for entity in entities {
                if *entity.spotted()
                    && timer.elapsed(Duration::from_millis(500))
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

#[derive(ContinuousPunishment)]
pub struct CursedSnipers {
    schedule: PunishmentSchedule,
}

impl Punishment for CursedSnipers {
    fn schedule(&self) -> &PunishmentSchedule {
        &self.schedule
    }

    fn action(
        &self,
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

#[derive(PeriodicPunishment)]
pub struct BunnyMan {
    schedule: PunishmentSchedule,
    timer: Mutex<Timer>,
}

impl Punishment for BunnyMan {
    fn schedule(&self) -> &PunishmentSchedule {
        &self.schedule
    }

    fn action(
        &self,
        modules: &HashMap<String, Module>,
        player: Option<&Player>,
        _: Option<&Vec<Entity>>,
    ) {
        let client = modules.get("client.dll").unwrap();
        let force_jump = client.address + offsets::buttons::jump;

        let Some(player) = player else {
            return;
        };

        let mut timer = self.timer.lock().unwrap();
        if timer.elapsed(Duration::from_millis(80)) {
            if player.is_grounded() {
                // process.write::<i32>(force_jump, Modifier::Plus as i32)?;
            } else {
                // process.write::<i32>(force_jump, Modifier::Minus as i32)?;
            }
        }
    }
}
