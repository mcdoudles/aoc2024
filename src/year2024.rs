pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

/// This is just something to pasta...
pub use crate::read_file;

pub fn first(_input: &str) -> i32 {
    0
}

pub fn second(_input: &str) -> i32 {
    0
}

#[test]
fn test_first() {
    assert_eq!(0, first(""));

    assert_eq!(
        0,
        first(read_file("resources/2024-1.txt".to_string()).as_str())
    );
}

#[test]
fn test_second() {
    assert_eq!(0, second(""));

    assert_eq!(
        0,
        second(read_file("resources/2024-1.txt".to_string()).as_str())
    );
}
