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

fn check(line: &str) -> Result<Vec<Bracket>, Bracket> {
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
    Ok(stack)
}

fn score(bracket: Bracket) -> usize {
    match bracket {
        Bracket::Round => 3,
        Bracket::Square => 57,
        Bracket::Curly => 1197,
        Bracket::Angle => 25137,
    }
}

fn score_autocomplete(stack: Vec<Bracket>) -> usize {
    stack
        .iter()
        .enumerate()
        .map(|(ix, bracket)| {
            5_usize.pow(ix.try_into().unwrap())
                * match bracket {
                    Bracket::Round => 1,
                    Bracket::Square => 2,
                    Bracket::Curly => 3,
                    Bracket::Angle => 4,
                }
        })
        .sum::<usize>()
}

pub fn main() {
    let input = include_str!("input/puzzle10");
    println!(
        "{}",
        input
            .lines()
            .filter_map(|l| check(l).err())
            .map(score)
            .sum::<usize>()
    );
    let mut scores: Vec<usize> = input
        .lines()
        .filter_map(|l| check(l).ok())
        .map(score_autocomplete)
        .collect();
    scores.sort();
    println!("{}", scores[scores.len() / 2]);
}

#[test]
fn test_bracket_check() {
    assert_eq!(check("()"), Ok(vec![]));
    assert_eq!(check("[]"), Ok(vec![]));
    assert_eq!(check("([])"), Ok(vec![]));
    assert_eq!(check("{()()()}"), Ok(vec![]));
    assert_eq!(check("<([{}])>"), Ok(vec![]));
    assert_eq!(check("[<>({}){}[([])<>]]"), Ok(vec![]));
    assert_eq!(check("(((((((((())))))))))"), Ok(vec![]));

    assert_eq!(check("(]"), Err(Bracket::Square));
    assert_eq!(check("{()()()>"), Err(Bracket::Angle));
    assert_eq!(check("(((()))}"), Err(Bracket::Curly));
    assert_eq!(check("<([]){()}[{}])"), Err(Bracket::Round));

    use Bracket::*;
    assert_eq!(
        check("[({(<(())[]>[[{[]{<()<>>"),
        Ok(vec![
            Square, Round, Curly, Round, Square, Square, Curly, Curly
        ])
    );
    assert_eq!(
        check("[(()[<>])]({[<{<<[]>>("),
        Ok(vec![Round, Curly, Square, Angle, Curly, Round])
    );
    assert_eq!(
        check("(((({<>}<{<{<>}{[]{[]{}"),
        Ok(vec![
            Round, Round, Round, Round, Angle, Curly, Angle, Curly, Curly
        ])
    );
    assert_eq!(
        check("{<[[]]>}<{[{[{[]{()[[[]"),
        Ok(vec![
            Angle, Curly, Square, Curly, Square, Curly, Curly, Square, Square
        ])
    );
    assert_eq!(
        check("<{([{{}}[<[[[<>{}]]]>[]]"),
        Ok(vec![Angle, Curly, Round, Square])
    );
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

#[test]
fn part2_example() {
    let mut scores: Vec<usize> = EXAMPLE_INPUT
        .lines()
        .filter_map(|line| check(line).ok())
        .map(score_autocomplete)
        .collect();
    assert_eq!(scores, vec![288957, 5566, 1480781, 995444, 294]);
    scores.sort();
    assert_eq!(scores[scores.len() / 2], 288957);
}
