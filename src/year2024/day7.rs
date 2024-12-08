pub use crate::read_file;

pub fn first(input: &str) -> i64 {
    let equations: Vec<Vec<&str>> = input.split("\n").map(|x| x.split(": ").collect()).collect();
    let mut total = 0;

    for equation in equations {
        println!("{} - {}", equation[0], equation[1]);
        let result: i64 = equation[0].parse().unwrap();
        let numbers: Vec<i64> = equation[1]
            .split(" ")
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect();

        for mut i in 0..2_u32.pow(u32::try_from(numbers.len() - 1).unwrap()) {
            let mut numbers = numbers.iter();
            let mut test_result: i64 = *numbers.next().unwrap();
            for n in numbers {
                if i & 1 == 0 {
                    test_result += n;
                } else {
                    test_result *= n;
                }
                i >>= 1;
            }

            if test_result == result {
                total += result;
                break;
            }
        }
    }

    total
}

pub fn second(input: &str) -> i64 {
    let equations: Vec<Vec<&str>> = input.split("\n").map(|x| x.split(": ").collect()).collect();
    let mut total = 0;

    for equation in equations {
        println!("{} - {}", equation[0], equation[1]);
        let result: i64 = equation[0].parse().unwrap();
        let numbers: Vec<i64> = equation[1]
            .split(" ")
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect();

        for mut i in 0..3_u32.pow(u32::try_from(numbers.len() - 1).unwrap()) {
            let mut numbers = numbers.iter();
            let mut test_result = *numbers.next().unwrap();
            for n in numbers {
                if i % 3 == 0 {
                    test_result += n;
                } else if i % 3 == 1 {
                    test_result *= n;
                } else {
                    test_result *= 10_i64.pow(n.ilog10() + 1);
                    test_result += n;
                }
                i /= 3;
            }

            if test_result == result {
                total += result;
                break;
            }
        }
    }

    total
}

#[test]
fn test_first() {
    assert_eq!(
        3749,
        first(
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
        )
    );

    assert_eq!(
        267566105056,
        first(read_file("resources/2024-7.txt".to_string()).as_str())
    );
}

#[test]
fn test_second() {
    assert_eq!(
        11387,
        second(
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
        )
    );

    assert_eq!(
        116094961956019,
        second(read_file("resources/2024-7.txt".to_string()).as_str())
    );
}
