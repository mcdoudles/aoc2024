pub use crate::read_file;

pub fn first() {
    let content = read_file("resources/2024-2.txt".to_string());

    let lines = content.split("\n");

    let mut total: i32 = 0;

    for line in lines {
        let numbers: Vec<i32> = line.split(" ").map(|x| x.trim().parse().unwrap()).collect();

        if is_safe(&numbers) {
            total = total + 1;
        }
    }

    println!("{}", total)
}

pub fn second() {
    let content = read_file("resources/2024-2.txt".to_string());

    let lines = content.split("\n");

    let mut total: i32 = 0;

    for line in lines {
        let numbers: Vec<i32> = line.split(" ").map(|x| x.trim().parse().unwrap()).collect();

        if is_safe(&numbers) {
            total = total + 1;
        } else {
            for x in 0..numbers.len() {
                let new_numbers: Vec<i32> = numbers
                    .iter().enumerate()
                    .filter(|(index, _)| *index != x).map(|(_, number)| number)
                    .cloned()
                    .collect();
                if is_safe(&new_numbers) {
                    total = total + 1;
                    break;
                }
            }
        }
    }

    println!("{}", total)
}

fn is_safe(numbers: &[i32]) -> bool {
    let mut increasing = false;

    if numbers[0] > numbers[1] {
        increasing = true;
    }

    let size = numbers.len();

    let mut valid = true;

    for x in 1..size {
        let difference = numbers[x] - numbers[x - 1];
        let positive = difference > 0;

        if difference == 0
            || (increasing && positive)
            || (!increasing && !positive)
            || difference.abs() > 3
        {
            valid = false;
            break;
        }
    }
    return valid;
}
