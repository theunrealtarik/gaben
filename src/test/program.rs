use std::thread;
use std::time::Duration;

fn main() {
    let mut health = 100;

    loop {
        println!("health {:#?} {:p}", health, &health);
        thread::sleep(Duration::from_secs(1));
    }
}
