pub mod old_day1;

pub mod year2024;

use std::fs;

fn main() {
}

pub fn read_file(filename: String) -> String {
    println!("Reading file {filename}");
    let content = fs::read_to_string(filename).expect("Unable to read file");
    content
}