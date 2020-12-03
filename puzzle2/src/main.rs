use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path;

#[macro_use]
extern crate scan_fmt;

#[derive(Debug)]
struct PasswordRule {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl PasswordRule {
    fn is_valid(&self, part2: bool) -> bool {
        if part2 {
            let first = self.password.chars().nth(self.min - 1).unwrap();
            let second = self.password.chars().nth(self.max - 1).unwrap();
            (first == self.letter) != (second == self.letter)
        } else {
            let letter = String::from(self.letter);
            let count = self.password.matches(&letter).count();
            count >= self.min && count <= self.max
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut part2 = false;
    let default = String::from("1");
    let arg = args.get(1).unwrap_or(&default);
    if arg == "2" {
        part2 = true;
    }
    let count = read_lines("input")
        .expect("Bad file")
        .map(|s| parse_line(&s.expect("Bad line in file")))
        .filter(|rule| rule.is_valid(part2))
        .count();
    println!("{}", count);
}

fn parse_line(line: &str) -> PasswordRule {
    let (min, max, letter, password) =
        scan_fmt!(&line, "{d}-{d} {}: {}", usize, usize, char, String).expect("Bad format");
    PasswordRule {
        min,
        max,
        letter,
        password: String::from(password),
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<path::Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
