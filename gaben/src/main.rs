mod punishments;

use punishments::*;
use sdk::prelude::*;

use rand::Rng;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[cfg(target_os = "windows")]
fn main() -> Result<(), anyhow::Error> {
    Keyboard::listen();

    if let Ok(process) = Memory::new(CS_PROCESS_NAME) {
        let process = Arc::new(process);

        let (ptx, prx) = mpsc::channel::<Arc<Player>>();
        let (etx, erx) = mpsc::channel::<Arc<Vec<Entity>>>();
        {
            let process = Arc::clone(&process);
            thread::spawn(move || {
                let modules = &process.modules;
                let mut rng = rand::thread_rng();
                let mut timer = Timer::default();
                let mut periodic = Punishments::new();

                periodic.add(Box::new(BunnyMan::new()));

                loop {
                    let player = prx.recv().ok();
                    let entities = erx.recv().ok();

                    if timer.elapsed(Duration::from_secs(60 * 2)) {
                        if timer.elapsed(Duration::from_secs(rng.gen_range(15..=30))) {
                            periodic.next().action(
                                &process,
                                player.as_deref(),
                                entities.as_deref(),
                            );
                        }
                    }
                }
            });
        }

        let client = process.modules.get("client.dll").unwrap();
        let mut continuous = Punishments::new();

        continuous.add(Box::new(SlippyWippyWeapon::new()));
        continuous.add(Box::new(CursedSnipers::new()));

        loop {
            let Ok(local_player) = process.read::<usize>(client.address + offsets::DW_LOCAL_PAWN)
            else {
                continue;
            };

            let Some(player) = Player::new(&process, local_player) else {
                continue;
            };

            let Some(entities) = Entity::get_entities(&process, client) else {
                continue;
            };

            for punishment in continuous.values() {
                punishment.action(&process, Some(&player), Some(&entities));
            }

            ptx.send(Arc::new(player)).unwrap();
            etx.send(Arc::new(entities)).unwrap();
        }
    }

    Ok(())
}
