use macros::*;
use rand::prelude::*;
use sdk::prelude::*;
use std::{
    sync::{Arc, Mutex}, // fearless concurrency mentioned letsgooo!!!!
    time::Duration,
};

/// Responsible for scheduling periodic punishments
pub struct PeriodicPunishments {
    punishments: Punishments,
    span_timer: Timer,
    interval_timer: Timer,
    busy: bool,
}

impl PeriodicPunishments {
    pub fn new() -> Self {
        Self {
            punishments: Punishments::new(),
            span_timer: Timer::default(),
            interval_timer: Timer::default(),
            busy: false,
        }
    }
}

impl PunishmentsExecutor for PeriodicPunishments {
    fn run(
        &mut self,
        process: Arc<Process>,
        player: Arc<Option<Player>>,
        entities: Arc<Option<Vec<Entity>>>,
    ) {
        let mut rng = rand::thread_rng();
        let mut span = self.span_timer;
        let mut interval = self.interval_timer;

        if interval.elapsed(Duration::from_secs(60 * 2)) {
            span.reset();
            self.busy = true;
            self.punishments.next();
        }

        if span.elapsed(Duration::from_secs(rng.gen_range(15..=30))) && self.busy {
            self.busy = false;
            if let Some(prev_punishment) = self.punishments.prev() {
                log::info!("withdraw {:?}", prev_punishment.name());
                prev_punishment.withdraw(&process, &player, &entities)
            }
        }

        if self.busy {
            if let Some(p) = self.punishments.prev() {
                p.action(&process, &player, &entities);
            }
        }
    }

    fn punishments(&self) -> &Punishments {
        &self.punishments
    }

    fn add(&mut self, p: Box<dyn Punishment>) {
        self.punishments.add(p);
    }
}

/// *Bunny Man*: when activated, it makes the cheater to constantly jump
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

    fn action(&self, process: &Process, player: &Option<Player>, _: &Option<Vec<Entity>>) {
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

const ZAZA_FOV: u32 = 200;
const DEFAULT_FOV: u32 = 68;
const CAMERA_SERVICES: usize = offsets::C_BasePlayerPawn::m_pCameraServices;
const I_FOV: usize = offsets::CCSPlayerBase_CameraServices::m_iFOV;

/// *Tianeptine*: sets the cheater's field of view to over the game's max which is 68 (close enough)
/// [ZAZA_FOV]
#[derive(PeriodicPunishment, Default)]
pub struct Tianeptine {
    schedule: PunishmentSchedule,
    name: String,
    default_fov: Mutex<Option<u32>>,
}

impl Punishment for Tianeptine {
    fn schedule(&self) -> &PunishmentSchedule {
        &self.schedule
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn action(&self, process: &Process, player: &Option<Player>, _: &Option<Vec<Entity>>) {
        let Some(player) = player else {
            return;
        };

        let Ok(camera_services) = process.read::<usize>(player.base_address() + CAMERA_SERVICES)
        else {
            return;
        };

        let mut default_fov = self.default_fov.lock().unwrap();
        if default_fov.is_none() {
            *default_fov = process.read::<u32>(camera_services + I_FOV).ok();
        }

        if !*player.is_scopped() {
            process
                .write(camera_services + I_FOV, ZAZA_FOV)
                .unwrap_or_else(|_| 0);
        }
    }

    fn withdraw(&self, process: &Process, player: &Option<Player>, _: &Option<Vec<Entity>>) {
        if let Some(player) = player {
            if let Ok(camera_services) =
                process.read::<usize>(player.base_address() + CAMERA_SERVICES)
            {
                let bytes = process.write(camera_services + I_FOV, DEFAULT_FOV);
                dbg!(&bytes);
            }
        }
    }
}