use itertools::Itertools;
use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path;

fn main() -> Result<(), io::Error> {
    let lines = read_lines("input")?;
    let seat_ids = lines.map(|s| code_to_seat_id(s.unwrap()));

    if is_part2() {
        let mut sorted: Vec<u16> = seat_ids.collect();
        sorted.sort_unstable();
        let (_, (seat_before, _)) = sorted
            .iter()
            .tuple_windows()
            .find_position(|seats: &(&u16, &u16)| *seats.1 != seats.0 + 1)
            .unwrap();
        println!("{}", seat_before + 1);
    } else {
        let largest_seat_id = seat_ids.max().unwrap();
        println!("{}", largest_seat_id);
    }

    Ok(())
}

fn code_to_seat_id(line: String) -> u16 {
    let maxbyte = line.len() - 1;
    line.bytes()
        .enumerate()
        .map(|(index, byte): (usize, u8)| {
            let digit = match byte {
                b'B' | b'R' => 1,
                _ => 0,
            };
            digit << (maxbyte - index)
        })
        .sum()
}

fn is_part2() -> bool {
    env::args().nth(1).map(|val| val == "2").unwrap_or(false)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<path::Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
