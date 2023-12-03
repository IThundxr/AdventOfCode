use std::time::SystemTime;

mod day1;
mod day2;

fn main() {
    // Start Time
    let time_start = SystemTime::now();

    day1::main();
    day2::main();

    // End Time
    let time_taken = SystemTime::now().duration_since(time_start).unwrap().as_micros();

    println!("\nFull program took a total of {}ms to complete", time_taken)
}