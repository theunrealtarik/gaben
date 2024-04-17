mod punishments;
use punishments::*;
use sdk::prelude::*;

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

#[cfg(target_os = "windows")]
fn main() -> Result<(), anyhow::Error> {
    Keyboard::listen();

    if let Ok(process) = Process::new(CS_PROCESS_NAME) {
        let process = Arc::new(process);

        let (ptx, prx) = mpsc::channel::<Arc<Option<Player>>>();
        let (etx, erx) = mpsc::channel::<Arc<Option<Vec<Entity>>>>();
        {
            let process = Arc::clone(&process);
            thread::spawn(move || PeriodicPunishments::run(process, prx, erx));
        }

        let client = process.modules.get("client.dll").unwrap();
        loop {
            let Ok(local_player) = process.read::<usize>(client.address + offsets::DW_LOCAL_PAWN)
            else {
                continue;
            };

            let entities = Arc::new(Entity::read_entities(&process, client));
            let player = Arc::new(Player::read(&process, local_player));

            ptx.send(Arc::clone(&player)).unwrap();
            etx.send(Arc::clone(&entities)).unwrap();

            ContinuousPunishments::run(&process, player, entities);
        }
    }

    Ok(())
}
