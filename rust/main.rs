use std::time::SystemTime;

fn main() {
    println!("Hello ASL!");
    let current_time = SystemTime::now();
    println!("{:?} ",current_time);
}
