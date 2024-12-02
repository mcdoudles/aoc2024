pub mod old_day1;
pub mod day1;
pub mod day2;

use std::fs;

use day2::{first, second};

fn main() {
    first();
    second();
}

pub fn read_file(filename: String) -> String {
    println!("Reading file {filename}");
    let content = fs::read_to_string(filename).expect("Unable to read file");
    content
}