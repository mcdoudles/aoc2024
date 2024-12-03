pub mod old_day1;
pub mod day1;
pub mod day2;
pub mod day3;

use std::fs;
use day3::{first, second};

fn main() {
    let binding = read_file("resources/2024-3.txt".to_string());
    let content = binding.as_str();

    println!("{}", first(content));
    println!("{}", second(content));
}

pub fn read_file(filename: String) -> String {
    println!("Reading file {filename}");
    let content = fs::read_to_string(filename).expect("Unable to read file");
    content
}