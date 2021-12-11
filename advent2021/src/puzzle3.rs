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

pub fn main(is_part2: bool) {
    assert!(!is_part2);
    let input = include_str!("input/puzzle3");
    let lines: Vec<&str> = input.lines().collect();
    let counts = count_bits(&lines);
    let (gamma, epsilon) = calc_power_consumption(&counts, lines.len());
    println!("{}", gamma * epsilon);
}

#[test]
fn part1_example() {
    const INPUT: [&'static str; 12] = [
        "00100",
        "11110",
        "10110",
        "10111",
        "10101",
        "01111",
        "00111",
        "11100",
        "10000",
        "11001",
        "00010",
        "01010",
    ];
    let counts = count_bits(&INPUT);
    assert_eq!(counts, [7, 5, 8, 7, 5]);
    let (gamma, epsilon) = calc_power_consumption(&counts, INPUT.len());
    assert_eq!(gamma, 22);
    assert_eq!(epsilon, 9);
}