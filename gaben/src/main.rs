mod punishments;
use punishments::*;
use sdk::logger;
use sdk::prelude::*;

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn attach(process: Process, window: Window) {
    let window = Arc::new(window);
    let process = Arc::new(process);

    let (ptx, prx) = mpsc::channel::<Arc<Option<Player>>>();
    let (etx, erx) = mpsc::channel::<Arc<Option<Vec<Entity>>>>();
    {
        let process = Arc::clone(&process);
        let window = Arc::clone(&window);
        thread::spawn(move || {
            let mut periodic = PeriodicPunishments::new();

            loop {
                if let (Ok(player), Ok(entities)) = (prx.recv(), erx.recv()) {
                    if !window.is_focused() {
                        continue;
                    }
                    periodic.run(Arc::clone(&process), player, entities);
                };
            }
        });
    }

    let client = process.modules.get("client.dll").unwrap();
    let mut continuous = ContinuousPunishments::new();

    loop {
        #[cfg(debug_assertions)]
        {
            if Key::LKey.is_pressed() {
                log::debug!("window focus {:?}", window.is_focused());
            }

            if Key::EndKey.is_pressed()
                || (!window.is_focused() && Key::LControlKey.is_pressed() && Key::CKey.is_pressed())
            {
                log::warn!("self killed");
                break;
            }
        }

        let Ok(local_player) = process.read::<usize>(client.address + offsets::DW_LOCAL_PAWN)
        else {
            continue;
        };

        let entities = Arc::new(Entity::read_entities(&process, client));
        let player = Arc::new(Player::read(&process, local_player));

        ptx.send(Arc::clone(&player)).unwrap();
        etx.send(Arc::clone(&entities)).unwrap();

        if window.is_focused() {
            continuous.run(Arc::clone(&process), player, entities);
        }
    }
}

#[cfg(target_os = "windows")]
fn main() {
    logger::init_env();
    Keyboard::listen();

    log::info!("monitoring ...");
    loop {
        thread::sleep(Duration::from_secs(1));

        let Ok(process) = Process::new(CS_PROCESS_NAME) else {
            log::error!("CS2 IS NOT RUNNING...");
            continue;
        };

        let Ok(window) = Window::find(CS_MAIN_WINDOW_NAME) else {
            continue;
        };

        log::info!("CS2 PROCESS WAS FOUND ({:?})", process.process_handle);
        log::info!("CS2 WINDOW WAS FOUND ({:?})", window.handle());

        attach(process, window);
    }
}
