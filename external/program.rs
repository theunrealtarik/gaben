use std::thread;
use std::time::Duration;

fn main() {
    let health = 100;
    println!("{:p}", &health);

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
