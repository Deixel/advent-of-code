use once_cell::sync::Lazy;
use regex::Regex;
use std::{fs, usize};

struct Round {
    red: usize,
    green: usize,
    blue: usize,
}

struct Game {
    id: usize,
    rounds: Vec<Round>,
    max_seen: Round,
    power: usize,
}

enum Colour {
    Red,
    Green,
    Blue,
}

impl Round {
    fn is_possible(&self, limit: &Round) -> bool {
        self.red <= limit.red && self.green <= limit.green && self.blue <= limit.blue
    }
}

fn main() {
    let file_path = "/home/deixel/Projects/advent_of_code/2023/aoc-2023-02/src/input.txt";

    let contents: String = fs::read_to_string(file_path).expect("Can't read the file");

    let lines = contents.split("\n");

    let limits = Round {
        red: 12,
        green: 13,
        blue: 14,
    };
    let mut total = 0;
    let mut power = 0;

    for line in lines {
        let game = parse_game(line);
        if game.max_seen.is_possible(&limits) {
            total += game.id;
        }
        power += game.power
    }

    println!("Total: {total}");
    println!("Power: {power}");
}

fn parse_game(line: &str) -> Game {
    let (game_number, end_prefix_byte) = get_game_number(line);
    let mut game = Game {
        id: game_number,
        rounds: vec![],
        max_seen: Round {
            red: 0,
            green: 0,
            blue: 0,
        },
        power: 0,
    };

    let (_, rounds_str) = line.split_at(end_prefix_byte);

    let rounds_str = rounds_str.split(";");
    for round in rounds_str {
        game.rounds.push(parse_round(round));
    }

    for round in &game.rounds {
        if game.max_seen.red < round.red {
            game.max_seen.red = round.red;
        }
        if game.max_seen.green < round.green {
            game.max_seen.green = round.green
        }
        if game.max_seen.blue < round.blue {
            game.max_seen.blue = round.blue
        }
    }
    game.power = game.max_seen.red * game.max_seen.green * game.max_seen.blue;

    game
}

fn get_game_number(line: &str) -> (usize, usize) {
    static GAME_NUM_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"^(?<title>Game (?<number>\d+):)").unwrap());

    let game_captures = GAME_NUM_REGEX.captures(line).unwrap();

    let game_number: usize = game_captures
        .name("number")
        .unwrap()
        .as_str()
        .parse()
        .unwrap();
    let end_byte = game_captures.name("title").unwrap().end();

    (game_number, end_byte)
}

fn parse_round(line: &str) -> Round {
    Round {
        red: parse_round_for_colour(line, Colour::Red),
        green: parse_round_for_colour(line, Colour::Green),
        blue: parse_round_for_colour(line, Colour::Blue),
    }
}

fn parse_round_for_colour(line: &str, colour: Colour) -> usize {
    static RED_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+) red").unwrap());
    static GREEN_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+) green").unwrap());
    static BLUE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+) blue").unwrap());

    let regex = match colour {
        Colour::Red => &RED_REGEX,
        Colour::Green => &GREEN_REGEX,
        Colour::Blue => &BLUE_REGEX,
    };

    let captures = regex.captures(line);
    let value = match captures {
        None => None,
        Some(cap) => Some(cap.get(1).unwrap().as_str()),
    };
    dbg!(value);
    match value {
        None => 0,
        Some(num_colour) => num_colour.parse().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colour() {
        let string = " 3 blue, 4 red";
        assert_eq!(3, parse_round_for_colour(string, Colour::Blue));
        assert_eq!(4, parse_round_for_colour(string, Colour::Red));
        assert_eq!(0, parse_round_for_colour(string, Colour::Green));
    }
}
