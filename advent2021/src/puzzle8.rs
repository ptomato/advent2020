use multimap::MultiMap;
use std::collections::HashMap;

fn process_signal(signal: &str) -> u8 {
    let mut retval = 0;
    signal.bytes().for_each(|l| match l {
        b'a' => retval |= 0b0000001,
        b'b' => retval |= 0b0000010,
        b'c' => retval |= 0b0000100,
        b'd' => retval |= 0b0001000,
        b'e' => retval |= 0b0010000,
        b'f' => retval |= 0b0100000,
        b'g' => retval |= 0b1000000,
        _ => panic!("unexpected signal character '{}'", l),
    });
    retval
}

fn process_line(line: &str) -> ([u8; 10], [u8; 4]) {
    let vals = scan_fmt!(
        line,
        &("{/[a-g]+/} ".repeat(10) + " | " + &"{/[a-g]+/} ".repeat(4)),
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String
    )
    .unwrap();
    (
        [
            vals.0, vals.1, vals.2, vals.3, vals.4, vals.5, vals.6, vals.7, vals.8, vals.9,
        ]
        .map(|s| process_signal(&s)),
        [vals.10, vals.11, vals.12, vals.13].map(|s| process_signal(&s)),
    )
}

fn count_unique_digits((_, output): ([u8; 10], [u8; 4])) -> usize {
    output
        .iter()
        .filter(|v| match v.count_ones() {
            2 | 3 | 4 | 7 => true,
            _ => false,
        })
        .count()
}

fn decode_digits((patterns, output): ([u8; 10], [u8; 4])) -> usize {
    let by_len = patterns
        .iter()
        .map(|&pattern| (pattern.count_ones(), pattern))
        .collect::<MultiMap<u32, u8>>();
    let mut mapping: HashMap<u8, usize> = HashMap::new();
    mapping.insert(by_len[&2], 1);
    mapping.insert(by_len[&3], 7);
    mapping.insert(by_len[&4], 4);
    for &five in by_len.get_vec(&5).unwrap() {
        mapping.insert(
            five,
            if five & by_len[&2] == by_len[&2] {
                3
            } else if (five & by_len[&4]).count_ones() == 3 {
                5
            } else {
                2
            },
        );
    }
    for &six in by_len.get_vec(&6).unwrap() {
        mapping.insert(
            six,
            if six & by_len[&2] != by_len[&2] {
                6
            } else if six & by_len[&4] == by_len[&4] {
                9
            } else {
                0
            },
        );
    }
    mapping.insert(by_len[&7], 8);
    mapping[&output[0]] * 1000
        + mapping[&output[1]] * 100
        + mapping[&output[2]] * 10
        + mapping[&output[3]]
}

pub fn main(is_part2: bool) {
    println!(
        "{}",
        include_str!("input/puzzle8")
            .lines()
            .map(process_line)
            .map(if is_part2 {
                decode_digits
            } else {
                count_unique_digits
            })
            .sum::<usize>()
    );
}

#[cfg(test)]
const EXAMPLE_LINE: &str = "\
acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";

#[test]
fn test_process_signal() {
    assert_eq!(process_signal("acedgfb"), 0b1111111);
    assert_eq!(process_signal("fbcad"), 0b0101111);
    assert_eq!(process_signal("cdbaf"), 0b0101111);
    assert_eq!(process_signal("ab"), 0b0000011);
    assert_eq!(process_signal("ba"), 0b0000011);
}

#[test]
fn test_process_line() {
    let (patterns, output) = process_line(EXAMPLE_LINE);
    assert_eq!(
        patterns,
        [
            0b1111111, 0b0111110, 0b1101101, 0b0101111, 0b0001011, 0b0111111, 0b1111110, 0b0110011,
            0b1011111, 0b0000011
        ]
    );
    assert_eq!(output, [0b0111110, 0b0101111, 0b0111110, 0b0101111]);
}

#[test]
fn test_decode() {
    let data = process_line(EXAMPLE_LINE);
    assert_eq!(decode_digits(data), 5353);
}

#[test]
fn part1_example() {
    assert_eq!(
        EXAMPLE_INPUT
            .lines()
            .map(process_line)
            .map(count_unique_digits)
            .sum::<usize>(),
        26
    );
}

#[test]
fn part2_example() {
    assert_eq!(
        EXAMPLE_INPUT
            .lines()
            .map(process_line)
            .map(decode_digits)
            .sum::<usize>(),
        61229
    );
}
