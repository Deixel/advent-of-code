use once_cell::sync::Lazy;
use regex::Regex;
use std::fs;

fn main() {
    let file_path = "/home/deixel/Projects/advent_of_code/2023/aoc-2023-01/src/input.txt";

    let contents: String = fs::read_to_string(file_path).expect("Can't read the file");

    let lines = contents.split("\n");

    let mut total = 0;
    for line in lines {
        let numbers_from_line = extract_numbers_from_line(line);
        total += numbers_from_line_to_number(numbers_from_line);
    }
    println!("Calibration: {total}"); // 56397
}

fn extract_numbers_from_line(line: &str) -> Vec<&str> {
    static numbers_regex: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d)").unwrap());

    let captured_numbers = numbers_regex.captures_iter(line);
    let mut numbers_from_line: Vec<&str> = vec![];
    for number in captured_numbers {
        let num_str = number.get(0).unwrap().as_str();

        numbers_from_line.push(num_str);
    }
    numbers_from_line
}

fn numbers_from_line_to_number(numbers_from_line: Vec<&str>) -> u32 {
    let first_number = numbers_from_line.first().unwrap();
    let last_number = numbers_from_line.last().unwrap();
    let mut final_number_str: String = first_number.to_string();
    final_number_str.push_str(last_number);

    final_number_str.parse().unwrap()
}
