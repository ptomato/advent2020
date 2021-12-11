use std::env;
use std::process;

#[macro_use] extern crate scan_fmt;

mod puzzle1;
mod puzzle2;

fn is_part2() -> bool {
    env::args().nth(2).map(|val| val == "2").unwrap_or(false)
}

fn main() {
    let puzzle = env::args().nth(1).unwrap_or_else(|| {
        println!("Requires an argument");
        process::exit(1);
    });
    match puzzle.as_str() {
        "1" => {puzzle1::main(is_part2());}
        "2" => {puzzle2::main(is_part2());}
        _ => {println!("Unknown puzzle {}", puzzle);}
    }
}