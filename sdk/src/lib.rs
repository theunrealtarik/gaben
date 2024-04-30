#[cfg(feature = "game")]
pub mod game;
#[cfg(feature = "inputs")]
pub mod inputs;
#[cfg(feature = "memory")]
pub mod memory;
#[cfg(feature = "offsets")]
pub mod offsets;
#[cfg(feature = "time")]
pub mod time;
#[cfg(feature = "types")]
pub mod types;

#[cfg(feature = "all")]
pub mod prelude {
    pub const CS_PROCESS_NAME: &str = "cs2.exe";
    pub const CS_MAIN_WINDOW_NAME: &str = "Counter-Strike 2";

    pub use super::game::*;
    pub use super::inputs::*;
    pub use super::memory::*;
    pub use super::offsets;
    pub use super::punishments::*;
    pub use super::time::*;
    pub use super::types::*;
    pub use super::utils::*;
}

#[cfg(feature = "punishments")]
pub mod punishments {
    use std::sync::Arc;

    use derive_getters::Getters;

    use super::game::*;
    use super::memory::*;

    #[derive(Default, Copy, Clone)]
    pub enum PunishmentSchedule {
        Periodic,
        #[default]
        Continuous,
    }

    pub trait Punishment: Send + Sync {
        fn schedule(&self) -> &PunishmentSchedule;
        fn name(&self) -> &String;
        fn action(
            &mut self,
            process: &Process,
            player: &Option<Player>,
            entities: &Option<Vec<Entity>>,
        );
        fn withdraw(
            &self,
            _process: &Process,
            _player: &Option<Player>,
            _entities: &Option<Vec<Entity>>,
        ) {
        }
    }

    /// A trait that must be implemented by a punishments-executor
    pub trait PunishmentsExecutor {
        fn run(
            &mut self,
            process: Arc<Process>,
            player: Arc<Option<Player>>,
            entities: Arc<Option<Vec<Entity>>>,
        );
        fn punishments(&self) -> &Punishments;
        fn add(&mut self, p: Box<dyn Punishment>);
    }

    #[derive(Getters, Default)]
    pub struct Punishments {
        elements: Vec<Box<dyn Punishment>>,
        curr_index: usize,
        prev_index: Option<usize>,
    }

    impl Punishments {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn add(&mut self, p: Box<dyn Punishment>) {
            self.elements.push(p);
        }

        pub fn next(&mut self) -> Option<&Box<dyn Punishment>> {
            let length = self.elements.len();

            if self.elements.is_empty() {
                return None;
            }

            let value = self.elements.get(self.curr_index);
            self.prev_index = Some(self.curr_index);
            self.curr_index = (self.curr_index + 1) % length;
            value
        }

        pub fn prev(&self) -> Option<&Box<dyn Punishment>> {
            match self.prev_index {
                Some(index) => self.elements.get(index),
                None => None,
            }
        }

        pub fn prev_mut(&mut self) -> Option<&mut Box<dyn Punishment>> {
            match self.prev_index {
                Some(index) => self.elements.get_mut(index),
                None => None,
            }
        }
    }
}

#[cfg(feature = "utils")]
pub mod utils {
    pub fn stringify_bytes_u8<T>(bytes: T) -> String
    where
        T: IntoIterator,
        T::Item: Into<u8>,
    {
        String::from_utf8(
            bytes
                .into_iter()
                .map(|i| {
                    let i: u8 = i.into();
                    i
                })
                .filter(|&i| i != 0)
                .collect::<Vec<u8>>(),
        )
        .unwrap_or_else(|_| String::new())
    }

    #[test]
    fn stringify_vec_bytes() {
        let bytes = vec![0x36, 0x39];
        let string = stringify_bytes_u8(bytes);
        assert_eq!(string, String::from("69"));
    }

    pub fn get_steam_id() -> Option<u32> {
        use winreg::enums::*;
        use winreg::RegKey;
        let hklm = RegKey::predef(HKEY_CURRENT_USER);
        if let Ok(active_process) = hklm.open_subkey("SOFTWARE\\Valve\\Steam\\ActiveProcess") {
            let Ok(id) = active_process.get_value::<u32, &str>("ActiveUser") else {
                return None;
            };

            return Some(id);
        }

        None
    }

    pub async fn send_message(webhook: &str, message: &str) {
        use reqwest::Client;
        use std::collections::HashMap;

        let mut body = HashMap::new();
        body.insert("content", message);

        match Client::new().post(webhook).json(&body).send().await {
            Ok(_) => log::info!("{:?}", body),
            Err(err) => log::error!("{:?}", err),
        };
    }
}

#[cfg(feature = "logger")]
pub mod logger {
    pub use env_logger;
    use env_logger::Env;
    pub use log;

    #[cfg(debug_assertions)]
    const DEBUG_TRACERS: &str = "sdk=trace,gaben=trace,pdm=trace";

    #[cfg(not(debug_assertions))]
    const DEBUG_TRACERS: &str = "";

    pub struct Logger;
    impl Logger {
        pub fn env() -> Env<'static> {
            let env = Env::default()
                .filter_or("RUST_LOG", DEBUG_TRACERS)
                .write_style_or("RUST_STYLE_LOG", "always");
            env
        }
    }

    pub fn init_env() {
        env_logger::init_from_env(Logger::env());
    }
}
