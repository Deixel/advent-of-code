use aoc_util::load_input;
use once_cell::sync::Lazy;
use regex::Regex;

struct Number {
    value: usize,
    length: usize,
}

enum Tile {
    Empty,
    Symbol,
    Number(Number),
}

fn main() {
    let lines = load_input("./src/input.txt");

    for line in lines {
        println!("{line}");
    }
}

fn is_symbol(str: &str) -> bool {
    static SYMBOL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^.\d]").unwrap());
    SYMBOL_REGEX.is_match(str)
}

fn search_left(line: &str, start_idx: usize, power: usize) -> usize {}
