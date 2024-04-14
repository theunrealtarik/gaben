pub mod game;
pub mod inputs;
pub mod memory;
pub mod offsets;
pub mod time;
pub mod types;

mod test;

#[allow(dead_code, unused_imports)]
pub mod prelude {
    pub const CS_PROCESS_NAME: &str = "cs2.exe";

    pub use super::game::*;
    pub use super::inputs::*;
    pub use super::memory::*;
    pub use super::offsets;
    pub use super::punishments;
    pub use super::time::*;
    pub use super::types::*;
    pub use super::utils::*;
}

pub mod punishments {
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
        fn action(&self, process: &Memory, player: Option<&Player>, entities: Option<&Vec<Entity>>);
    }

    #[derive(Getters)]
    pub struct Punishments {
        values: Vec<Box<dyn Punishment>>,
        index: usize,
    }

    impl Punishments {
        pub fn new() -> Self {
            Self {
                values: Vec::new(),
                index: 0,
            }
        }

        pub fn add(&mut self, p: Box<dyn Punishment>) {
            self.values.push(p);
        }

        pub fn next(&mut self) -> &Box<dyn Punishment> {
            let value = self.values.get(self.index);
            self.index = (self.index + 1) % self.values.len();

            let value = value.clone().unwrap();
            value
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
}
