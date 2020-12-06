use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path;

fn main() -> Result<(), io::Error> {
    let mut total = 0;
    let mut current = HashMap::new();
    let mut group_size = 0;
    for line in read_lines("input")?.map(|s| s.unwrap()) {
        if line == "" {
            total += count_answers(&current, group_size);
            current = HashMap::new();
            group_size = 0;
            continue;
        }
        for byte in line.bytes() {
            let count = current.entry(byte).or_insert(0);
            *count += 1;
        }
        group_size += 1;
    }
    total += count_answers(&current, group_size);
    println!("{}", total);

    Ok(())
}

fn count_answers(group: &HashMap<u8, usize>, group_size: usize) -> usize {
    if is_part2() {
        group
            .iter()
            .filter(|(_, count): &(&u8, &usize)| **count == group_size)
            .count()
    } else {
        group.len()
    }
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
