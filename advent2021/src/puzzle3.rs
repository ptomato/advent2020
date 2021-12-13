fn count_bits(input: &[&str]) -> Vec<usize> {
    let mut counts = vec![0; input[0].len()];
    for s in input.iter() {
        for (j, c) in s.bytes().enumerate() {
            counts[j] += match c {
                b'0' => 0,
                b'1' => 1,
                _ => panic!("illegal character {}", c),
            };
        }
    }
    counts
}

fn calc_power_consumption(counts: &[usize], n_bits: usize) -> (u64, u64) {
    let len = counts.len();
    let mut gamma = 0u64;
    for (ix, &count) in counts.iter().enumerate() {
        if count > n_bits - count {
            gamma |= 1u64 << (len - ix - 1);
        }
    }
    let epsilon = !gamma & ((1u64 << len) - 1);
    (gamma, epsilon)
}

fn oxygen_bit(count: usize, len: usize) -> bool {
    count >= len - count
}

fn co2_bit(count: usize, len: usize) -> bool {
    count < len - count
}

fn calc_rating(lines: &[&str], bit_func: &impl Fn(usize, usize) -> bool) -> u64 {
    let mut possibilities = Vec::from(lines);
    for count_index in 0..possibilities[0].len() {
        let count = count_bits(&possibilities)[count_index];
        let bit = if bit_func(count, possibilities.len()) {
            b'1'
        } else {
            b'0'
        };
        possibilities.retain(|val| val.as_bytes()[count_index] == bit);
        if possibilities.len() == 1 {
            break;
        }
    }
    u64::from_str_radix(possibilities[0], 2).unwrap()
}

pub fn main(is_part2: bool) {
    let input = include_str!("input/puzzle3");
    let lines: Vec<&str> = input.lines().collect();
    if is_part2 {
        let oxygen = calc_rating(&lines, &oxygen_bit);
        let co2 = calc_rating(&lines, &co2_bit);
        println!("{}", oxygen * co2);
    } else {
        let counts = count_bits(&lines);
        let (gamma, epsilon) = calc_power_consumption(&counts, lines.len());
        println!("{}", gamma * epsilon);
    }
}

#[cfg(test)]
static EXAMPLE_INPUT: [&'static str; 12] = [
    "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
    "00010", "01010",
];

#[test]
fn part1_example() {
    let counts = count_bits(&EXAMPLE_INPUT);
    assert_eq!(counts, [7, 5, 8, 7, 5]);
    let (gamma, epsilon) = calc_power_consumption(&counts, EXAMPLE_INPUT.len());
    assert_eq!(gamma, 22);
    assert_eq!(epsilon, 9);
}

#[test]
fn part2_example() {
    let oxygen = calc_rating(&EXAMPLE_INPUT, &oxygen_bit);
    assert_eq!(oxygen, 23);
    let co2 = calc_rating(&EXAMPLE_INPUT, &co2_bit);
    assert_eq!(co2, 10);
}
