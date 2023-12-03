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
        total += numbers_from_line_to_final_number(numbers_from_line);
    }
    println!("Calibration: {total}");
}

fn extract_numbers_from_line(line: &str) -> Vec<&str> {
    static NUMBERS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d)").unwrap());

    let captured_numbers = NUMBERS_REGEX.captures_iter(line);
    let mut numbers_from_line: Vec<&str> = vec![];
    for number in captured_numbers {
        let num_str = number.get(0).unwrap().as_str();

        numbers_from_line.push(num_str);
    }
    numbers_from_line
}

fn numbers_from_line_to_final_number(numbers_from_line: Vec<&str>) -> u32 {
    let first_number = numbers_from_line.first().unwrap();
    let last_number = numbers_from_line.last().unwrap();
    let mut final_number_str: String = first_number.to_string();
    final_number_str.push_str(last_number);

    final_number_str.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{extract_numbers_from_line, numbers_from_line_to_final_number};

    #[test]
    fn test_number_extraction() {
        assert_eq!(vec!["1", "2"], extract_numbers_from_line("1abc2"));
        assert_eq!(vec!["3", "8"], extract_numbers_from_line("pqr3stu8vwx"));
        assert_eq!(vec!["1", "2", "3","4", "5"], extract_numbers_from_line("a1b2c3d4e5f"));
        assert_eq!(vec!["7"], extract_numbers_from_line("treb7uchet"));
    }

    #[test]
    fn test_first_and_last_number() {
        assert_eq!(12, numbers_from_line_to_final_number(vec!["1", "2"]));
        assert_eq!(38, numbers_from_line_to_final_number(vec!["3", "8"]));
        assert_eq!(15, numbers_from_line_to_final_number(vec!["1", "2", "3","4", "5"]));
        assert_eq!(77, numbers_from_line_to_final_number(vec!["7"]));
    }
}
