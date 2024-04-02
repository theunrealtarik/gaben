use delegate::delegate;
use enigo::{self, Enigo, Key, KeyboardControllable};
use inputbot;

#[allow(unused_imports)]
pub use inputbot::{MouseButton, MouseCursor, MouseWheel};

pub type LisKey = inputbot::KeybdKey;
pub type EmuKey = Key;

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

    delegate! {
        to self.enigo {
            pub fn key_click(&mut self, key: Key);
        }
    }
}
