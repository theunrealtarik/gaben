use std::sync::Mutex;
use std::time::Duration;

use macros::PeriodicPunishment;
use sdk::prelude::*;

#[derive(PeriodicPunishment, Default)]
pub struct BunnyMan {
    schedule: PunishmentSchedule,
    timer: Mutex<Timer>,
    name: String,
}

impl Punishment for BunnyMan {
    fn schedule(&self) -> &PunishmentSchedule {
        &self.schedule
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn action(&self, process: &Memory, player: &Option<Player>, _: &Option<Vec<Entity>>) {
        let client = process.modules.get("client.dll").unwrap();
        let force_jump = client.address + offsets::buttons::jump;

        let Some(player) = player else {
            return;
        };

        let mut timer = self.timer.lock().unwrap();
        if timer.elapsed(Duration::from_millis(80)) {
            if player.is_grounded() {
                process
                    .write::<i32>(force_jump, Modifier::Plus as i32)
                    .unwrap();
            } else {
                process
                    .write::<i32>(force_jump, Modifier::Minus as i32)
                    .unwrap();
            }
        }
    }
}
