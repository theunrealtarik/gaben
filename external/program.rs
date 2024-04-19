use std::thread;
use std::time::Duration;

fn main() {
    let the_name = "agen";
    let health = 100;

    println!("{:p}", &the_name);
    println!("{:p}", &health);

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
