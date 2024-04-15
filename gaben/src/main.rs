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

        let (ptx, prx) = mpsc::channel::<Arc<Option<Player>>>();
        let (etx, erx) = mpsc::channel::<Arc<Option<Vec<Entity>>>>();
        {
            let process = Arc::clone(&process);
            thread::spawn(move || {
                let modules = &process.modules;
                let mut rng = rand::thread_rng();
                let mut periodic = Punishments::new();

                periodic.add(Box::new(BunnyMan::new()));

                let mut sc_timer = Timer::default();
                let mut tf_timer = Timer::default();
                let mut triggered = false;

                loop {
                    let player = prx.recv().ok().unwrap();
                    let entities = erx.recv().ok().unwrap();

                    if sc_timer.elapsed(Duration::from_secs(60)) {
                        println!("triggered");
                        tf_timer.reset();
                        triggered = true;
                    }

                    if tf_timer.elapsed(Duration::from_secs(rng.gen_range(15..=30))) && triggered {
                        triggered = false
                    } else if triggered {
                        let p = periodic.next();
                        p.action(&process, &player, &entities);
                    }
                }
            });
        }

        let client = process.modules.get("client.dll").unwrap();
        let mut continuous = Punishments::new();

        continuous.add(Box::new(SlipperyWeapons::new()));
        continuous.add(Box::new(CursedSnipers::new()));
        continuous.add(Box::new(FragileTrigger::new()));

        loop {
            let Ok(local_player) = process.read::<usize>(client.address + offsets::DW_LOCAL_PAWN)
            else {
                continue;
            };

            let entities = Entity::read_entities(&process, client);
            let player = Player::read(&process, local_player);

            for punishment in continuous.values() {
                punishment.action(&process, &player, &entities);
            }

            ptx.send(Arc::new(player)).unwrap();
            etx.send(Arc::new(entities)).unwrap();
        }
    }

    Ok(())
}
