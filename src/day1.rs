use std::time::SystemTime;

pub fn main() {
    let time_start = SystemTime::now();

    let input = include_str!("../input/day1/day1.txt");

    // Code Here :)

    let time_taken = SystemTime::now().duration_since(time_start).unwrap().as_micros();

    println!("Day 1: Not done :(, Took a total of {}ms to complete", time_taken)
}