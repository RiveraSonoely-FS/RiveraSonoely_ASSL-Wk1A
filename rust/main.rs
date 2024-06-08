use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
    let seconds = current_time.as_secs();
    let milliseconds = current_time.subsec_millis();

    println!("Hello ASL! ", (seconds * 1000) + u64::from(milliseconds));
}