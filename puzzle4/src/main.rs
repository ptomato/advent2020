use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path;

#[macro_use]
extern crate lazy_static;

fn main() {
    let mut passports = vec![];
    let mut current = HashMap::new();
    for line in read_lines("input").map(|s| s.expect("Bad line in file")) {
        if line == "" {
            passports.push(current);
            current = HashMap::new();
        }
        current.extend(get_pairs_from_line(&line).drain());
    }
    passports.push(current);

    let count = passports.iter().filter(passport_is_valid).count();
    println!("{}", count);
}

fn passport_is_valid(passport: &&HashMap<String, String>) -> bool {
    let n_keys = passport.len();
    (n_keys == 8 || (n_keys == 7 && !passport.contains_key("cid")))
        && (!is_part2()
            || valid_birth_year(&passport["byr"])
                && valid_issue_year(&passport["iyr"])
                && valid_expiry_year(&passport["eyr"])
                && valid_height(&passport["hgt"])
                && valid_hair_color(&passport["hcl"])
                && valid_eye_color(&passport["ecl"])
                && valid_passport_id(&passport["pid"]))
}

fn valid_birth_year(byr: &str) -> bool {
    valid_year(byr, 1920, 2002)
}

fn valid_issue_year(iyr: &str) -> bool {
    valid_year(iyr, 2010, 2020)
}

fn valid_expiry_year(eyr: &str) -> bool {
    valid_year(eyr, 2020, 2030)
}

fn valid_year(y: &str, min: u16, max: u16) -> bool {
    lazy_static! {
        static ref YEAR_REGEX: Regex = Regex::new(r"^[12]\d{3}$").unwrap();
    }
    if !YEAR_REGEX.is_match(y) {
        return false;
    }
    let year = y.parse::<u16>().unwrap();
    year >= min && year <= max
}

fn valid_height(hgt: &str) -> bool {
    lazy_static! {
        static ref HEIGHT_REGEX: Regex = Regex::new(r"^(\d{2,3})(cm|in)$").unwrap();
    }
    let groups = match HEIGHT_REGEX.captures(hgt) {
        Some(groups) => groups,
        None => return false,
    };
    let height = groups[1].parse::<u8>().unwrap();
    match &groups[2] {
        "cm" => height >= 150 && height <= 193,
        "in" => height >= 59 && height <= 76,
        _ => false,
    }
}

fn valid_hair_color(hcl: &str) -> bool {
    lazy_static! {
        static ref HAIR_REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }
    HAIR_REGEX.is_match(hcl)
}

fn valid_eye_color(ecl: &str) -> bool {
    match ecl {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false,
    }
}

fn valid_passport_id(pid: &str) -> bool {
    lazy_static! {
        static ref ID_REGEX: Regex = Regex::new(r"^\d{9}$").unwrap();
    }
    ID_REGEX.is_match(pid)
}

fn get_pairs_from_line(line: &str) -> HashMap<String, String> {
    let mut new_pairs = HashMap::new();
    for pair in line.split_whitespace() {
        new_pairs.insert(String::from(&pair[..3]), String::from(&pair[4..]));
    }
    new_pairs
}

fn is_part2() -> bool {
    match env::args().nth(1) {
        Some(s) if s == "2" => true,
        _ => false,
    }
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<fs::File>>
where
    P: AsRef<path::Path>,
{
    let file = fs::File::open(filename).expect("Bad file");
    io::BufReader::new(file).lines()
}
