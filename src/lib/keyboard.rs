use ambassador::{delegatable_trait_remote, Delegate};
use enigo::{self, Enigo, Key};
use inputbot;

pub use enigo::KeyboardControllable;

#[allow(unused_imports)]
pub use inputbot::{MouseButton, MouseCursor, MouseWheel};

pub type LisKey = inputbot::KeybdKey;
pub type EmuKey = Key;

#[delegatable_trait_remote]
pub trait KeyboardControllable {
    fn key_sequence(&mut self, sequence: &str);
    fn key_down(&mut self, key: EmuKey);
    fn key_up(&mut self, key: EmuKey);
    fn key_click(&mut self, key: EmuKey);
}

#[derive(Delegate)]
#[delegate(KeyboardControllable)]
pub struct Keyboard {
    enigo: Enigo,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            enigo: Enigo::new(),
        }
    }

    pub fn listen(&self) {
        std::thread::spawn(move || {
            inputbot::handle_input_events();
        });
    }
}
