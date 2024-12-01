pub use crate::read_file;

pub fn first() {
    let content = read_file("resources/2024-1.txt".to_string());
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    for line in content.split("\n") {
        let mut splitted = line.split_whitespace();
        left_list.push(splitted.next().expect("Error").parse().unwrap());
        right_list.push(splitted.next().expect("Error").parse().unwrap());
    }

    left_list.sort();
    right_list.sort();

    let n = left_list.len();

    let mut total: i32 = 0;

    for i in 0..n {
        println!("{}-{}", left_list[i], right_list[i]);
        total = total + (left_list[i] - right_list[i]).abs();
    }

    println!("{}", total)
}

pub fn second() {
    let content = read_file("resources/2024-1.txt".to_string());
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    for line in content.split("\n") {
        let mut splitted = line.split_whitespace();
        left_list.push(splitted.next().expect("Error").parse().unwrap());
        right_list.push(splitted.next().expect("Error").parse().unwrap());
    }

    let mut total: i32 = 0;

    for number in left_list {
        total = total + number * (right_list.iter().filter(|x| **x == number).count() as i32)
    }

    println!("{}", total)
}