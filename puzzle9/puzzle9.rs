use itertools::Itertools;
use std::fs;
use std::io::{self, BufRead};

fn main() -> Result<(), io::Error> {
    let file = fs::File::open("input")?;
    let numbers: Vec<u64> = read_lines(file).map(|s| s.parse().unwrap()).collect();

    let answer = invalid_number(&numbers, 25).unwrap();

    println!("Part 1: {}", answer);
    println!("Part 2: {}", encryption_weakness(&numbers, answer).unwrap());

    Ok(())
}

fn invalid_number(numbers: &[u64], window_size: usize) -> Option<u64> {
    for window in numbers.windows(window_size + 1) {
        let preamble = &window[..window_size];
        let number = window[window_size];
        if !preamble
            .iter()
            .tuple_combinations()
            .any(|(a, b)| a + b == number)
        {
            return Some(number);
        }
    }
    None
}

fn encryption_weakness(numbers: &[u64], invalid_number: u64) -> Option<u64> {
    for window_size in 2..numbers.len() {
        for window in numbers.windows(window_size) {
            if window.iter().sum::<u64>() == invalid_number {
                let (largest, smallest) = window.iter().minmax().into_option().unwrap();
                return Some(largest + smallest);
            }
        }
    }
    None
}

fn read_lines(file: fs::File) -> impl Iterator<Item = String> {
    io::BufReader::new(file).lines().map(|res| res.unwrap())
}
