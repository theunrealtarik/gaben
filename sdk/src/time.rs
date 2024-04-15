use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

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

    pub fn reset(&mut self) {
        self.0 = Instant::now();
    }
}

pub struct Timers<T>(pub HashMap<T, Timer>)
where
    T: std::hash::Hash + std::cmp::Eq + std::cmp::PartialEq;

impl<T> Timers<T>
where
    T: std::hash::Hash + std::cmp::Eq + std::cmp::PartialEq,
{
    pub fn elapsed(&mut self, tag: T, duration: Duration) -> bool {
        match self.0.get_mut(&tag) {
            Some(timer) => {
                if timer.elapsed(duration) {
                    self.0.insert(tag, Timer(Instant::now()));
                    return true;
                }

                false
            }
            None => {
                self.0.insert(tag, Timer(Instant::now()));
                false
            }
        }
    }
}