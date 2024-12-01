pub use crate::read_file;

pub fn first() {
    let content = read_file("resources/2028-1.txt".to_string());
    let mut total: i32 = 0;
    for line in content.split("\n") {
        let number : i32 = line.trim().parse().unwrap();
        total = total + number;
    }
    println!("{total}")
}