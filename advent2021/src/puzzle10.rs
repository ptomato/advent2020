#[derive(Debug, PartialEq)]
enum Bracket {
    Round,
    Square,
    Curly,
    Angle,
}

fn expect(stack: &mut Vec<Bracket>, expected: Bracket) -> Result<(), Bracket> {
    if stack.pop().unwrap() == expected {
        Ok(())
    } else {
        Err(expected)
    }
}

fn check(line: &str) -> Result<(), Bracket> {
    let mut stack: Vec<Bracket> = Default::default();
    for b in line.bytes() {
        match b {
            b'(' => stack.push(Bracket::Round),
            b'[' => stack.push(Bracket::Square),
            b'{' => stack.push(Bracket::Curly),
            b'<' => stack.push(Bracket::Angle),
            b')' => expect(&mut stack, Bracket::Round)?,
            b']' => expect(&mut stack, Bracket::Square)?,
            b'}' => expect(&mut stack, Bracket::Curly)?,
            b'>' => expect(&mut stack, Bracket::Angle)?,
            _ => panic!("Unexpected character {}", b),
        }
    }
    Ok(())
}

fn score(bracket: Bracket) -> usize {
    match bracket {
        Bracket::Round => 3,
        Bracket::Square => 57,
        Bracket::Curly => 1197,
        Bracket::Angle => 25137,
    }
}

pub fn main(_is_part2: bool) {
    println!(
        "{}",
        include_str!("input/puzzle10")
            .lines()
            .filter_map(|l| check(l).err())
            .map(score)
            .sum::<usize>()
    );
}

#[test]
fn test_bracket_check() {
    assert_eq!(check("()"), Ok(()));
    assert_eq!(check("[]"), Ok(()));
    assert_eq!(check("([])"), Ok(()));
    assert_eq!(check("{()()()}"), Ok(()));
    assert_eq!(check("<([{}])>"), Ok(()));
    assert_eq!(check("[<>({}){}[([])<>]]"), Ok(()));
    assert_eq!(check("(((((((((())))))))))"), Ok(()));

    assert_eq!(check("(]"), Err(Bracket::Square));
    assert_eq!(check("{()()()>"), Err(Bracket::Angle));
    assert_eq!(check("(((()))}"), Err(Bracket::Curly));
    assert_eq!(check("<([]){()}[{}])"), Err(Bracket::Round));
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";

#[test]
fn part1_example() {
    assert_eq!(
        EXAMPLE_INPUT
            .lines()
            .filter_map(|line| check(line).err())
            .map(score)
            .sum::<usize>(),
        26397
    );
}
