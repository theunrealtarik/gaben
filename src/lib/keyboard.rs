use inputbot;

#[allow(unused_imports)]
pub use inputbot::{KeybdKey, MouseButton, MouseCursor, MouseWheel};

pub struct Keyboard;

impl Keyboard {
    pub fn listen() {
        std::thread::spawn(move || {
            inputbot::handle_input_events();
        });
    }
}
