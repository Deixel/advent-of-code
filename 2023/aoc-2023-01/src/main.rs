use std::fs;
use regex::Regex;

fn main() {
    let file_path = "/home/deixel/Projects/advent_of_code/2023/aoc-2023-01/src/input.txt";

    let contents: String = fs::read_to_string(file_path).expect("Can't read the file");

    let lines = contents.split("\n");

    let regex = Regex::new(r"(\d)").unwrap();

    let mut numbers:Vec<Vec<&str>> = vec![];
    for line in lines {
        let captured_numbers = regex.captures_iter(line);
        let mut numbers_from_line: Vec<&str> = vec![];
            for number in captured_numbers {
            let num_str = number.get(0).unwrap().as_str();
            // let num = num_str.parse::<u32>().unwrap();

            numbers_from_line.push(num_str);
        }
        numbers.push(numbers_from_line);
    }

    let mut total = 0;
    for line in numbers {
        let first_number = line.first().unwrap();
        let last_number = line.last().unwrap();

        let mut final_number_str :String = first_number.to_string();
        final_number_str.push_str(last_number);

        let final_number: u32 = final_number_str.parse().unwrap();
        total += final_number;

    }

    println!("{total}");

}
