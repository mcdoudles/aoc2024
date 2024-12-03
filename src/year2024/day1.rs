pub use crate::read_file;

pub fn first(input: &str) -> i32 {
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    for line in input.split("\n") {
        let mut splitted = line.split_whitespace();
        left_list.push(splitted.next().expect("Error").parse().unwrap());
        right_list.push(splitted.next().expect("Error").parse().unwrap());
    }

    left_list.sort();
    right_list.sort();

    let n = left_list.len();

    let mut total: i32 = 0;

    for i in 0..n {
        total = total + (left_list[i] - right_list[i]).abs();
    }

    total
}

pub fn second(input: &str) -> i32 {
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    for line in input.split("\n") {
        let mut splitted = line.split_whitespace();
        left_list.push(splitted.next().expect("Error").parse().unwrap());
        right_list.push(splitted.next().expect("Error").parse().unwrap());
    }

    let mut total: i32 = 0;

    for number in left_list {
        total = total + number * (right_list.iter().filter(|x| **x == number).count() as i32)
    }

    total
}

#[test]
fn test_first() {
    assert_eq!(
        11,
        first(
            "3   4
4   3
2   5
1   3
3   9
3   3"
        )
    );

    assert_eq!(
        1938424,
        first(read_file("resources/2024-1.txt".to_string()).as_str())
    );
}

#[test]
fn test_second() {
    assert_eq!(
        31,
        second(
            "3   4
4   3
2   5
1   3
3   9
3   3"
        )
    );

    assert_eq!(
        22014209,
        second(read_file("resources/2024-1.txt".to_string()).as_str())
    );
}
