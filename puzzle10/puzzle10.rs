use itertools::Itertools;
use std::env;
use std::fs;
use std::io::{self, BufRead};

fn main() -> Result<(), io::Error> {
    let file = fs::File::open("input")?;
    let mut adapters: Vec<u8> = read_lines(file).map(|s| s.parse().unwrap()).collect();
    adapters.push(0); // add charging outlet
    adapters.sort_unstable();
    adapters.push(adapters.last().unwrap() + 3); // add built-in adapter

    let differences = adapters.iter().tuple_windows().map(|(j1, j2)| j2 - j1);

    if is_part2() {
        let groups = differences.group_by(|d| *d);
        let total: u64 = groups
            .into_iter()
            .filter(|(key, _)| *key == 1)
            .map(|(_, group)| possible_configurations(group.count()))
            .product();
        println!("{}", total);
    } else {
        let mut ones = 0;
        let mut threes = 0;
        for difference in differences {
            match difference {
                1 => ones += 1,
                3 => threes += 1,
                _ => (),
            }
        }
        println!("{}", ones * threes);
    }

    Ok(())
}

fn possible_configurations(run_length: usize) -> u64 {
    match run_length {
        n if n < 1 => panic!("Bad value"),
        1 => 1,
        2 => 2,
        3 => 4,
        4 => 7,
        n => 2 * possible_configurations(n - 1) - possible_configurations(n - 4),
    }
}

fn is_part2() -> bool {
    env::args().nth(1).map(|val| val == "2").unwrap_or(false)
}

fn read_lines(file: fs::File) -> impl Iterator<Item = String> {
    io::BufReader::new(file).lines().map(|res| res.unwrap())
}
