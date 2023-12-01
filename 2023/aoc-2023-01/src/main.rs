use std::fs;

fn main() {
    let file_path = "/home/deixel/Projects/advent_of_code/2023/aoc-2023-01/src/input.txt";

    let contents = fs::read_to_string(file_path).expect("Can't read the file");

    let lines = contents.split("\n");

    for line in lines {
        println!("{line}");
    }
}
