use std::collections::BTreeMap;

pub use crate::read_file;

pub fn first(input: &str) -> i32 {
    let splitted: Vec<&str> = input.split("\n\n").collect();

    let rules: Vec<(&str, &str)> = splitted[0]
        .split("\n")
        .map(|x| x.split("|").collect::<Vec<&str>>())
        .map(|vec| (vec[0], vec[1]))
        .collect();
    let mut updates: BTreeMap<usize, Vec<(&str, &str)>> = BTreeMap::new();

    let mut total = 0;

    for (line_number, line) in splitted[1].split("\n").enumerate() {
        let instructions: Vec<&str> = line.split(",").collect();
        let mut vec: Vec<(&str, &str)> = vec![];
        for i in 0..instructions.len() {
            let first = instructions[i];
            for j in i + 1..instructions.len() {
                vec.push((first, instructions[j]));
            }
            updates.insert(line_number, vec.clone());
        }
    }

    for update in updates {
        let mut correct = true;
        for instruction in update.1 {
            if !rules.contains(&instruction) {
                correct = false;
                break;
            }
        }

        if correct {
            let find: Vec<&str> = splitted[1].split("\n").collect::<Vec<&str>>()[update.0]
                .split(",")
                .collect();
            total = total + find[find.len() / 2].parse::<i32>().unwrap();
        }
    }
    total
}

pub fn second(_input: &str) -> i32 {
    0
}

#[test]
fn test_first() {
    assert_eq!(
        143,
        first(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
        )
    );

    assert_eq!(
        4185,
        first(read_file("resources/2024-5.txt".to_string()).as_str())
    );
}

#[test]
fn test_second() {
    assert_eq!(123, second("47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"));

    assert_eq!(
        0,
        second(read_file("resources/2024-5.txt".to_string()).as_str())
    );
}
