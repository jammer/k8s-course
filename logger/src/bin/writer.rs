use chrono::prelude::*;
use std::fs;
use std::{thread, time};

fn main() -> std::io::Result<()> {
    loop {
        let time = Utc::now();
        let resp = format!("{:?}",time);
        println!("{:?}",resp);
        fs::write("/data/time.txt",resp + "\n")?;
        let five_seconds = time::Duration::from_secs(5);
        thread::sleep(five_seconds);
    }
}

