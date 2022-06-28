use chrono::prelude::*;
use uuid::Uuid;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let id = Uuid::new_v4();
    loop {
        let time = Utc::now();
        println!("{:?} {}", time, id);
        sleep(Duration::from_secs(5));
    }
}
