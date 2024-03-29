use std::env;
use std::process;

#[macro_use]
extern crate scan_fmt;

mod puzzle1;
mod puzzle10;
mod puzzle2;
mod puzzle3;
mod puzzle4;
mod puzzle5;
mod puzzle6;
mod puzzle7;
mod puzzle8;
mod puzzle9;

fn is_part2() -> bool {
    env::args().nth(2).map(|val| val == "2").unwrap_or(false)
}

fn main() {
    let puzzle = env::args().nth(1).unwrap_or_else(|| {
        println!("Requires an argument");
        process::exit(1);
    });
    match puzzle.as_str() {
        "1" => puzzle1::main(is_part2()),
        "2" => puzzle2::main(is_part2()),
        "3" => puzzle3::main(is_part2()),
        "4" => puzzle4::main(is_part2()),
        "5" => puzzle5::main(is_part2()),
        "6" => puzzle6::main(is_part2()),
        "7" => puzzle7::main(is_part2()),
        "8" => puzzle8::main(is_part2()),
        "9" => puzzle9::main(),
        "10" => puzzle10::main(),
        _ => println!("Unknown puzzle {}", puzzle),
    }
}
