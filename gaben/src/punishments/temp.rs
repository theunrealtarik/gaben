use macros::*;
use rand::prelude::*;
use sdk::prelude::*;
use std::{
    sync::{Arc, Mutex}, // fearless concurrency mentioned letsgooo!!!!
    time::Duration,
};

#[cfg(debug_assertions)]
const PUNISHMENT_INTERVAL_DURATION: Duration = Duration::from_secs(30);
#[cfg(debug_assertions)]
const PUNISHMENT_SPAN_MIN_DURATION: u64 = 5;
#[cfg(debug_assertions)]
const PUNISHMENT_SPAN_MAX_DURATION: u64 = 10;

#[cfg(not(debug_assertions))]
const PUNISHMENT_INTERVAL_DURATION: Duration = Duration::from_secs(60 * 4);
#[cfg(not(debug_assertions))]
const PUNISHMENT_SPAN_MIN_DURATION: u64 = 15;
#[cfg(not(debug_assertions))]
const PUNISHMENT_SPAN_MAX_DURATION: u64 = 30;

/// Responsible for scheduling periodic punishments
pub struct PeriodicPunishments {
    punishments: Punishments,
    span_timer: Timer,
    interval_timer: Timer,
    rng: ThreadRng,
    busy: bool,
}

impl PeriodicPunishments {
    pub fn new() -> Self {
        let mut punishments = Punishments::new();
        punishments.add(Box::new(BunnyMan::new()));
        punishments.add(Box::new(Tianeptine::new()));

        Self {
            punishments,
            span_timer: Timer::default(),
            interval_timer: Timer::default(),
            rng: rand::thread_rng(),
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
        let interval = &mut self.interval_timer;
        let span = &mut self.span_timer;

        if interval.elapsed(PUNISHMENT_INTERVAL_DURATION) {
            span.reset();
            self.busy = true;
            if let Some(p) = self.punishments.next() {
                log::debug!("elapsed on {:?}", p.name());
            };
        }

        if span
            .elapsed(Duration::from_secs(self.rng.gen_range(
                PUNISHMENT_SPAN_MIN_DURATION..=PUNISHMENT_SPAN_MAX_DURATION,
            )))
            && self.busy
        {
            self.busy = false;
            if let Some(prev_punishment) = self.punishments.prev() {
                log::debug!("withdraw {:?}", prev_punishment.name());
                prev_punishment.withdraw(&process, &player, &entities)
            }
        }

        if self.busy {
            if let Some(p) = self.punishments.prev_mut() {
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

    fn action(&mut self, process: &Process, player: &Option<Player>, _: &Option<Vec<Entity>>) {
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

const DEFAULT_FOV: u32 = 68;
const CAMERA_SERVICES: usize = offsets::C_BasePlayerPawn::m_pCameraServices;
const I_FOV: usize = offsets::CCSPlayerBase_CameraServices::m_iFOV;

/// *Tianeptine*: sets the cheater's field of view to over the game's max which is 68 (close enough)
/// [ZAZA_FOV]
#[derive(PeriodicPunishment, Default)]
pub struct Tianeptine {
    schedule: PunishmentSchedule,
    name: String,
}

impl Punishment for Tianeptine {
    fn schedule(&self) -> &PunishmentSchedule {
        &self.schedule
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn action(&mut self, process: &Process, player: &Option<Player>, _: &Option<Vec<Entity>>) {
        let Some(player) = player else {
            return;
        };

        let Ok(camera_services) = process.read::<usize>(player.base_address() + CAMERA_SERVICES)
        else {
            return;
        };

        if *player.is_scopped() || !*player.is_alive() {
            return;
        }

        process.write::<u32>(camera_services + I_FOV, 200).unwrap();
    }

    fn withdraw(&self, process: &Process, player: &Option<Player>, _: &Option<Vec<Entity>>) {
        if let Some(player) = player {
            if let Ok(camera_services) =
                process.read::<usize>(player.base_address() + CAMERA_SERVICES)
            {
                process.write(camera_services + I_FOV, DEFAULT_FOV).unwrap();
            }
        }
    }
}
