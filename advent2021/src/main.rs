use std::env;
use std::process;

mod puzzle1;

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
        _ => {println!("Unknown puzzle {}", puzzle);}
    }
}
