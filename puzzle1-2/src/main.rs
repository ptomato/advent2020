use itertools::Itertools;
use std::fs;
use std::io::{self, BufRead};
use std::path;
use std::process;

fn main() {
    let entries: Vec<i32> = read_lines("input")
        .expect("Bad file")
        .map(|s| s.expect("Bad line in file").parse::<i32>().unwrap())
        .collect();
    for (first, second, third) in entries.iter().tuple_combinations() {
        if first + second + third == 2020 {
            println!(
                "{} × {} × {} = {}",
                first,
                second,
                third,
                first * second * third
            );
            process::exit(0);
        }
    }
    println!("Not found");
    process::exit(1);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<path::Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
