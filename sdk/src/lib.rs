pub mod game;
pub mod inputs;
pub mod memory;
pub mod offsets;
pub mod time;
pub mod types;

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
            &self,
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
    }
}

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
}

pub mod logger {
    pub use env_logger;
    use env_logger::Env;

    pub struct Logger;
    impl Logger {
        pub fn env() -> Env<'static> {
            let env = Env::default()
                .filter_or("RUST_LOG", "sdk=trace,gaben=trace")
                .write_style_or("RUST_STYLE_LOG", "always");
            env
        }
    }

    pub fn init_env() {
        env_logger::init_from_env(Logger::env());
    }
}
