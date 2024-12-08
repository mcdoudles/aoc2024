use std::collections::HashSet;

pub use crate::read_file;

pub fn first(input: &str) -> i32 {
    let map: Vec<Vec<&str>> = input.split("\n").map(|x| x.split("").collect()).collect();

    let mut position: (usize, usize, &str) = (0, 0, "");

    'outer: for (row_index, row) in map.iter().enumerate() {
        for (column_index, char) in row.iter().enumerate() {
            if char == &"^" || char == &"<" || char == &">" || char == &"v" {
                position = (row_index, column_index, char);
                break 'outer;
            }
        }
    }

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    loop {
        println!(
            "{}-({},{},{})",
            map[position.0][position.1], position.0, position.1, position.2
        );

        if position.2 == "<" {
            for i in (0..(position.1)).rev() {
                if map[position.0][i] == "#" {
                    position = (position.0, i + 1, "^");
                    break;
                } else if i == 0 {
                    visited.insert((position.0, i));
                    return visited.iter().count() as i32;
                } else {
                    position = (position.0, i, "<");
                    visited.insert((position.0, i));
                }
            }
        } else if position.2 == ">" {
            for i in position.1..map.len() + 1 {
                if map[position.0][i] == "#" {
                    position = (position.0, i - 1, "v");
                    break;
                } else if i == map.len() {
                    return visited.iter().count() as i32;
                } else {
                    position = (position.0, i, ">");
                    visited.insert((position.0, i));
                }
            }
        } else if position.2 == "^" {
            for i in (0..(position.0)).rev() {
                if map[i][position.1] == "#" {
                    position = (i + 1, position.1, ">");
                    break;
                } else if i == 0 {
                    visited.insert((position.0, i));
                    return visited.iter().count() as i32;
                } else {
                    position = (i, position.1, "^");
                    visited.insert((i, position.1));
                }
            }
        } else if position.2 == "v" {
            for i in position.0..map.len() + 1 {
                if i == map.len() {
                    return visited.iter().count() as i32;
                } else if map[i][position.1] == "#" {
                    position = (i - 1, position.1, "<");
                    break;
                } else {
                    position = (i, position.1, "v");
                    visited.insert((i, position.1));
                }
            }
        }
    }
}

pub fn second(_input: &str) -> i32 {
    0
}

#[test]
fn test_first() {
    assert_eq!(
        41,
        first(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
        )
    );

    assert_eq!(
        4656,
        first(read_file("resources/2024-6.txt".to_string()).as_str())
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
