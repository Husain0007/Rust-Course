use std::thread;
use std::time::Duration;

fn simulated_Expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly ...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {
    println!("Hello, world!");
}
