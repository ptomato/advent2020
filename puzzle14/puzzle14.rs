use std::collections::HashMap;
use std::convert::TryInto;
use std::env;
use std::error::Error;
use std::fs;
use std::io::{self, BufRead};
use std::num;

#[macro_use]
extern crate scan_fmt;

fn main() -> Result<(), Box<dyn Error>> {
    let mut memory = HashMap::new();
    let mut or_mask: u64 = 0;
    let mut and_mask: u64 = u64::MAX;
    let mut float_mask: u64 = 0;

    let file = fs::File::open("input")?;
    for line in read_lines(file) {
        if line.starts_with("mask") {
            let (new_or_mask, new_and_mask, new_float_mask) = parse_mask(&line[7..])?;
            or_mask = new_or_mask;
            and_mask = new_and_mask;
            float_mask = new_float_mask;
            continue;
        }
        let (addr, value) = scan_fmt!(&line, "mem[{}] = {}", u64, u64)?;
        if is_part2() {
            write_floating_memory(&mut memory, addr | or_mask, value, float_mask);
        } else {
            memory.insert(addr, value & and_mask | or_mask);
        }
    }

    println!("{}", memory.values().sum::<u64>());

    Ok(())
}

fn parse_mask(line: &str) -> Result<(u64, u64, u64), num::TryFromIntError> {
    let mut or_mask: u64 = 0;
    let mut and_mask: u64 = u64::MAX;
    let mut float_mask: u64 = 0;

    for (ix, byte) in line.bytes().rev().enumerate() {
        let bit = 2_u64.pow(ix.try_into()?);
        match byte {
            b'0' => and_mask -= bit,
            b'1' => or_mask += bit,
            b'X' => float_mask += bit,
            _ => panic!("Bad byte {}", byte),
        }
    }
    Ok((or_mask, and_mask, float_mask))
}

fn write_floating_memory(memory: &mut HashMap<u64, u64>, addr: u64, value: u64, float_mask: u64) {
    for mut floating_bits in 0..2_u64.pow(float_mask.count_ones()) {
        let mut masked_addr = addr;
        for bit_ix in 0..36 {
            let bit = 2_u64.pow(bit_ix);
            if float_mask & bit != 0 {
                match floating_bits & 1 {
                    0 => masked_addr &= !bit,
                    1 => masked_addr |= bit,
                    _ => panic!("Not possible"),
                };
                floating_bits >>= 1;
            }
        }
        memory.insert(masked_addr, value);
    }
}

fn is_part2() -> bool {
    env::args().nth(1).map(|val| val == "2").unwrap_or(false)
}

fn read_lines(file: fs::File) -> impl Iterator<Item = String> {
    io::BufReader::new(file).lines().map(|res| res.unwrap())
}
