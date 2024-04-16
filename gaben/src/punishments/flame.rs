use macros::*;
use sdk::prelude::*;
use std::thread;
use std::time::Duration;

#[derive(Default, ContinuousPunishment)]
pub struct FlameGrantMeStrength {
    name: String,
    schedule: PunishmentSchedule,
    timer: Timer,
}

impl Punishment for FlameGrantMeStrength {
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

        let client = process.modules.get("client.dll").unwrap();
        if *player.is_alive() && (player.weapon().is_molotov() || player.weapon().is_incendiary()) {
            process
                .write(client.address + offsets::buttons::attack2, Modifier::Plus)
                .unwrap();
            thread::sleep(Duration::from_millis(80));
            process
                .write(client.address + offsets::buttons::attack2, Modifier::Minus)
                .unwrap();
        }
    }
}
