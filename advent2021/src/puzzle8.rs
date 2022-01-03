fn process_line(line: &str) -> ([String; 10], [String; 4]) {
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
        ],
        [vals.10, vals.11, vals.12, vals.13],
    )
}

fn count_unique_digits((_, output): ([String; 10], [String; 4])) -> usize {
    output
        .iter()
        .filter(|v| match v.len() {
            2 | 3 | 4 | 7 => true,
            _ => false,
        })
        .count()
}

pub fn main(_is_part2: bool) {
    println!(
        "{}",
        include_str!("input/puzzle8")
            .lines()
            .map(process_line)
            .map(count_unique_digits)
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
fn test_process_line() {
    let (patterns, output) = process_line(EXAMPLE_LINE);
    assert_eq!(
        patterns,
        ["acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab"]
    );
    assert_eq!(output, ["cdfeb", "fcadb", "cdfeb", "cdbaf"]);
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
