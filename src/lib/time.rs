use std::time::{Duration, Instant};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Timer(Instant);

impl Default for Timer {
    fn default() -> Self {
        Self(Instant::now())
    }
}

#[allow(dead_code)]
impl Timer {
    pub fn once(&self, duration: Duration) -> bool {
        let now = Instant::now();
        let delta = now - self.0;
        delta > duration
    }

    pub fn elapsed(&mut self, duration: Duration) -> bool {
        let now = Instant::now();
        let delta = now - self.0;
        if delta > duration {
            self.0 = Instant::now();
            true
        } else {
            false
        }
    }
}
