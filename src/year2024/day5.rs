pub use crate::read_file;

pub fn first(input: &str) -> i32 {
    let splitted: Vec<&str> = input.split("\n\n").collect();
    let rules: Vec<(&str, &str)> = splitted[0]
        .split("\n")
        .map(|x| x.split_once("|").unwrap())
        .collect();
    let updates: &Vec<(usize, &str)> = &splitted[1].split("\n").enumerate().collect();
    let mut lookup: &str = "";

    let mut indexes: Vec<usize> = vec![];

    for (i, update) in updates {
        for page in update.split(",") {
            if lookup == "" {
                lookup = page;
                continue;
            }
            if !rule_exist(page, lookup, &rules) {
                break;
            }
        }

        indexes.push(*i);
    }

    let mut total : i32 = 0;
    for index in indexes {
        let update : Vec<&str> = updates
            .iter()
            .find(|x| x.0 == index)
            .map(|x| x.1)
            .unwrap().split(",").collect();
        total = total + update[update.len() / 2].parse::<i32>().unwrap();
    }
        total
}

fn rule_exist(page: &str, lookup: &str, rules: &Vec<(&str, &str)>) -> bool {
    rules
        .iter()
        .find(|(from, to)| from == &page && to == &lookup)
        .is_some()
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
        0,
        first(read_file("resources/2024-5.txt".to_string()).as_str())
    );
}

#[test]
fn test_second() {
    assert_eq!(0, second(""));

    assert_eq!(
        0,
        second(read_file("resources/2024-5.txt".to_string()).as_str())
    );
}
