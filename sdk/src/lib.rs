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
    use super::game::*;
    use super::memory::*;
    use std::collections::HashMap;

    #[derive(Default)]
    pub enum PunishmentSchedule {
        Periodic,
        #[default]
        Continous,
    }

    pub trait Punishment {
        fn schedule(&self) -> &PunishmentSchedule;
        fn action(
            &mut self,
            modules: &HashMap<String, Module>,
            player: Option<&Player>,
            entities: Option<&Vec<Entity>>,
        );
    }

    pub struct Punishments<T>
    where
        T: Punishment,
    {
        periodic: Vec<T>,
        continuous: Vec<T>,
    }

    impl<T> Punishments<T>
    where
        T: Punishment,
    {
        pub fn new() -> Self {
            Self {
                periodic: Vec::new(),
                continuous: Vec::new(),
            }
        }

        pub fn add(mut self, punishment: T) -> Self {
            match punishment.schedule() {
                PunishmentSchedule::Periodic => self.periodic.push(punishment),
                PunishmentSchedule::Continous => self.continuous.push(punishment),
            }
            self
        }

        pub fn run(
            mut self,
            modules: &HashMap<String, Module>,
            player: Option<&Player>,
            entities: Option<&Vec<Entity>>,
        ) {
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
