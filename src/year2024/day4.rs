use std::collections::BTreeMap;

use fancy_regex::Regex;

pub use crate::read_file;

pub fn first(input: &str) -> i32 {
    let horizontal: BTreeMap<usize, String> = input
        .split("\n")
        .enumerate()
        .map(|(i, x)| (i, x.to_string()))
        .collect();
    let chars: Vec<Vec<&str>> = input
        .split("\n")
        .map(|x| x.split("").filter(|x| *x != "").collect())
        .collect();

    let mut vertical: BTreeMap<usize, String> = BTreeMap::new();
    let mut diagonal_l_r: BTreeMap<i32, String> = BTreeMap::new();
    let mut diagonal_r_l: BTreeMap<i32, String> = BTreeMap::new();

    for i in 0..horizontal.len() {
        for j in 0..horizontal.len() {
            vertical.insert(
                j,
                vertical
                    .get(&j)
                    .or(Some(&"".to_string()))
                    .expect("Error")
                    .to_owned()
                    + chars[i][j],
            );
            let diagonal_l_r_key = i as i32 - j as i32;
            diagonal_l_r.insert(
                diagonal_l_r_key,
                diagonal_l_r
                    .get(&diagonal_l_r_key)
                    .or(Some(&"".to_string()))
                    .expect("Error")
                    .to_owned()
                    + chars[i][j],
            );

            let diagonal_r_l_key = i as i32 + j as i32;
            diagonal_r_l.insert(
                diagonal_r_l_key,
                diagonal_r_l
                    .get(&diagonal_r_l_key)
                    .or(Some(&"".to_string()))
                    .expect("Error")
                    .to_owned()
                    + chars[i][j],
            );
        }
    }

    let mut total: usize = 0;

    for h in horizontal {
        total = total + check_line(h.1);
    }

    println!();

    for v in vertical {
        total = total + check_line(v.1);
    }

    println!();

    for d in diagonal_l_r {
        total = total + check_line(d.1);
    }

    println!();

    for d in diagonal_r_l {
        total = total + check_line(d.1);
    }

    println!("{}", total);

    total as i32
}

pub fn check_line(line: String) -> usize {
    let xmas_regex = Regex::new(r"XMAS").unwrap();
    let samx_regex = Regex::new(r"SAMX").unwrap();

    let xmas_count = xmas_regex.find_iter(&line).count();
    let samx_count = samx_regex.find_iter(&line).count();
    println!("{} --> ({}, {})", line, xmas_count, samx_count);
    xmas_count + samx_count
}

pub fn second(input: &str) -> i32 {
    let horizontal: BTreeMap<usize, String> = input
        .split("\n")
        .enumerate()
        .map(|(i, x)| (i, x.to_string()))
        .collect();

    let chars: Vec<Vec<&str>> = input
        .split("\n")
        .map(|x| x.split("").filter(|x| *x != "").collect())
        .collect();

    let mut total = 0;

    for i in 1..horizontal.len() - 1 {
        for j in 1..horizontal.len() - 1 {
            if chars[i][j] == "A" {
                let tl = chars[i - 1][j - 1];
                let tr = chars[i - 1][j + 1];
                let bl = chars[i + 1][j - 1];
                let br = chars[i + 1][j + 1];

                if [tl, tr, bl, br].iter().filter(|x| **x == "M").count() == 2
                    && [tl, tr, bl, br].iter().filter(|x| **x == "S").count() == 2
                    && tl != br
                    && tr != bl
                {
                    total = total + 1;
                }
            }
        }
    }
    total
}

#[test]
fn test_check_line() {
    assert_eq!(2, check_line("MAMXMASAMX".to_string()))
}

#[test]
fn test_first() {
    assert_eq!(
        18,
        first(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
        )
    );

    assert_eq!(
        2532,
        first(read_file("resources/2024-4.txt".to_string()).as_str())
    );
}

#[test]
fn test_second() {
    assert_eq!(
        9,
        second(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
        )
    );

    assert_eq!(
        1941,
        second(read_file("resources/2024-4.txt".to_string()).as_str())
    );
}
