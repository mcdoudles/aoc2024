use regex::Regex;

pub fn first(input: &str) -> i32 {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();

    let mut result: i32 = 0;

    for split in re.find_iter(input).map(|x| x.as_str()) {
        result = result + multiply(split);
    }

    result
}

pub fn second(input: &str) -> i32 {
    let re = Regex::new(r"(mul\(\d+,\d+\)|do\(\)|don't\(\))").unwrap();
    let relevant_matches: Vec<&str> = re.find_iter(input).map(|x| x.as_str()).collect();

    let mut result: i32 = 0;
    let mut is_do: bool = true;

    for s in relevant_matches {
        if s.starts_with("mul") && is_do {
            result = result + multiply(s);
        } else if s == "do()" {
            is_do = true;
        } else if s == "don't()" {
            is_do = false;
        }
    }
    result
}

pub fn multiply(mul: &str) -> i32 {
    let mult: Vec<i32> = mul
        .replace("mul", "")
        .replace("(", "")
        .replace(")", "")
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    mult[0] * mult[1]
}

#[test]
fn test_first() {
    assert_eq!(
        161,
        first("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")
    )
}

#[test]
fn test_second() {
    assert_eq!(
        48,
        second("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))")
    )
}
