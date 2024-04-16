use std::time::Duration;

#[allow(unused_imports)]
pub use inputbot::{BlockInput, MouseButton, MouseCursor, MouseWheel};

pub type Key = inputbot::KeybdKey;

pub struct Keyboard;

impl Keyboard {
    pub fn stroke(key: Key) {
        key.press();
        std::thread::sleep(Duration::from_millis(20));
        key.release();
    }

    pub fn press(key: Key) {
        key.press()
    }
    pub fn release(key: Key) {
        key.release()
    }

    pub fn block(key: Key) {
        key.block_bind(|| ());
    }

    pub fn listen() {
        std::thread::spawn(move || {
            inputbot::handle_input_events();
        });
    }
}

pub struct Mouse;

impl Mouse {
    pub fn click(button: MouseButton) {
        button.press();
        std::thread::sleep(Duration::from_millis(20));
        button.release();
    }
}
