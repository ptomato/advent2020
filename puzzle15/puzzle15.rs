use std::collections::HashMap;
use std::env;

fn main() {
    let input = vec![15, 12, 0, 14, 3, 1];
    let mut last_seen: HashMap<usize, usize> = input
        .iter()
        .enumerate()
        .map(|(turn, starting_number)| (*starting_number, turn))
        .collect();
    let mut last_turn_number = 0;

    for turn in (input.len() + 1)..if is_part2() { 30000000 } else { 2020 } {
        let this_turn_number = match last_seen.get(&last_turn_number) {
            Some(prev_seen) => turn - 1 - prev_seen,
            None => 0,
        };
        last_seen.insert(last_turn_number, turn - 1);
        last_turn_number = this_turn_number;

        println!("Turn {}: {}", turn + 1, this_turn_number);
    }
}

fn is_part2() -> bool {
    env::args().nth(1).map(|val| val == "2").unwrap_or(false)
}
