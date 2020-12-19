extern crate peg;

use std::env;

peg::parser! {
    grammar bizarro_arithmetic() for str {
        rule number() -> u64 = n:$(['0'..='9']) { n.parse().unwrap() }
        pub rule expr() -> u64 = precedence!{
            x:(@) " + " y:@ { x + y }
            x:(@) " * " y:@ { x * y }
            --
            n:number() { n }
            "(" e:expr() ")" { e }
        }
        pub rule expr2() -> u64 = precedence!{
            x:(@) " * " y:@ { x * y }
            --
            x:(@) " + " y:@ { x + y }
            --
            n:number() { n }
            "(" e:expr2() ")" { e }
        }
    }
}

fn main() {
    let input = include_str!("input");
    let answer: u64 = input
        .lines()
        .map(if is_part2() {
            bizarro_arithmetic::expr2
        } else {
            bizarro_arithmetic::expr
        })
        .map(|n| n.unwrap())
        .sum();
    println!("{}", answer);
}

fn is_part2() -> bool {
    env::args().nth(1).map(|val| val == "2").unwrap_or(false)
}

#[test]
fn part1_examples() {
    assert_eq!(bizarro_arithmetic::expr("1 + 2 * 3 + 4 * 5 + 6"), Ok(71));
    assert_eq!(
        bizarro_arithmetic::expr("1 + (2 * 3) + (4 * (5 + 6))"),
        Ok(51)
    );
    assert_eq!(bizarro_arithmetic::expr("2 * 3 + (4 * 5)"), Ok(26));
    assert_eq!(
        bizarro_arithmetic::expr("5 + (8 * 3 + 9 + 3 * 4 * 3)"),
        Ok(437)
    );
    assert_eq!(
        bizarro_arithmetic::expr("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
        Ok(12240)
    );
    assert_eq!(
        bizarro_arithmetic::expr("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        Ok(13632)
    );
}

#[test]
fn part2_examples() {
    assert_eq!(bizarro_arithmetic::expr2("1 + 2 * 3 + 4 * 5 + 6"), Ok(231));
    assert_eq!(
        bizarro_arithmetic::expr2("1 + (2 * 3) + (4 * (5 + 6))"),
        Ok(51)
    );
    assert_eq!(bizarro_arithmetic::expr2("2 * 3 + (4 * 5)"), Ok(46));
    assert_eq!(
        bizarro_arithmetic::expr2("5 + (8 * 3 + 9 + 3 * 4 * 3)"),
        Ok(1445)
    );
    assert_eq!(
        bizarro_arithmetic::expr2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
        Ok(669060)
    );
    assert_eq!(
        bizarro_arithmetic::expr2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        Ok(23340)
    );
}
