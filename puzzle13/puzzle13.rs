use std::env;
use std::error::Error;
use std::fs;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::File::open("input")?;
    let mut lines = read_lines(file);
    let arrival: u64 = lines.next().unwrap().parse()?;

    let table = lines.next().unwrap();
    let entries = table.split(',');

    if is_part2() {
        let mut t: u64 = 100000000000000;
        let mut constraints: Vec<(usize, u64)> = entries
            .enumerate()
            .filter_map(|(ix, s)| s.parse::<u64>().map(|n| (ix, n)).ok()) // (index, bus_number)
            .collect();
        constraints.sort_unstable_by_key(|(_, bus_number)| *bus_number);
        constraints.reverse();

        let mut step = 1;
        let mut constraints_satisfied = 0;
        loop {
            match constraints
                .iter()
                .position(|(delay, bus)| (t + *delay as u64) % *bus != 0)
            {
                None => break,
                Some(ix) => {
                    if ix > constraints_satisfied {
                        constraints_satisfied += 1;
                        step *= constraints[ix - 1].1;
                    }
                }
            }
            t += step;
        }

        println!("{}", t);
    } else {
        let (bus_number, wait_time) = entries
            .filter_map(|s| s.parse::<u64>().ok()) // available bus lines
            .map(|interval| (interval, interval - arrival % interval)) // (bus_number, wait time)
            .min_by_key(|(_, wait_time)| *wait_time)
            .unwrap();

        println!("{}", bus_number * wait_time);
    }
    Ok(())
}

fn is_part2() -> bool {
    env::args().nth(1).map(|val| val == "2").unwrap_or(false)
}

fn read_lines(file: fs::File) -> impl Iterator<Item = String> {
    io::BufReader::new(file).lines().map(|res| res.unwrap())
}
