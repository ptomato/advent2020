extern crate peg;

use regex::Regex;
use std::collections::HashMap;
use std::env;

#[derive(Debug, PartialEq)]
pub enum Rule {
    Literal(char),
    Ref(usize),
    Seq(Vec<Rule>),
    Choice(Box<Rule>, Box<Rule>),
}

type RuleSet = HashMap<usize, Rule>;

peg::parser! {
    grammar rules_grammar() for str {
        rule index() -> usize = n:$(['0'..='9']+) ":" { n.parse().unwrap() }
        rule number() -> Rule = n:$(['0'..='9']+) { Rule::Ref(n.parse().unwrap()) }
        rule literal() -> Rule = "\"" c:$(['a'..='z' | 'A'..='Z']) "\"" {
            Rule::Literal(c.chars().next().unwrap())
        }
        rule seq() -> Rule = refs:(number() **<2,> " ") { Rule::Seq(refs) }
        rule choice_side() -> Rule = seq() / number()
        rule choice() -> Rule = a:choice_side() " | " b:choice_side() {
            Rule::Choice(Box::new(a), Box::new(b))
        }
        pub rule parse_line() -> (usize, Rule) =
            ix:index() " " expr:(choice() / seq() / number() / literal()) { (ix, expr) }
    }
}

fn rule_to_regex(rule_set: &RuleSet, rule: &Rule, is_part2: bool) -> String {
    use Rule::*;
    match rule {
        Literal(c) => c.to_string(),
        Ref(ref_ix) => rule_index_to_regex(rule_set, *ref_ix, is_part2),
        Seq(refs) => refs
            .iter()
            .map(|r| rule_to_regex(rule_set, r, is_part2))
            .collect::<Vec<String>>()
            .join(""),
        Choice(l, r) => format!(
            "({}|{})",
            rule_to_regex(rule_set, &*l, is_part2),
            rule_to_regex(rule_set, &*r, is_part2)
        ),
    }
}

fn rule_index_to_regex(rule_set: &RuleSet, ix: usize, is_part2: bool) -> String {
    let rule = rule_set.get(&ix).unwrap();
    match ix {
        8 if is_part2 => format!("({})+", rule_to_regex(rule_set, rule, is_part2)),
        11 if is_part2 => {
            if let Rule::Seq(refs) = rule {
                format!(
                    "({0}{1}|{0}{{2}}{1}{{2}}|{0}{{3}}{1}{{3}}|{0}{{4}}{1}{{4}})",
                    rule_to_regex(rule_set, &refs[0], is_part2),
                    rule_to_regex(rule_set, &refs[1], is_part2)
                )
            } else {
                panic!("Wrong type for rule 11");
            }
        }
        _ => rule_to_regex(rule_set, rule, is_part2),
    }
}

fn rule_set_to_regex(rule_set: &RuleSet, is_part2: bool) -> String {
    format!("^{}$", rule_index_to_regex(rule_set, 0, is_part2))
}

fn main() {
    let input = include_str!("input");
    let mut blocks = input.split("\n\n");

    let rules_block = blocks.next().unwrap();
    let mut rule_set = RuleSet::new();
    for line in rules_block.lines() {
        let (ix, rule) = rules_grammar::parse_line(line).unwrap();
        rule_set.insert(ix, rule);
    }
    let matcher = Regex::new(&rule_set_to_regex(&rule_set, is_part2())).unwrap();

    let messages_block = blocks.next().unwrap();
    let matches = messages_block
        .lines()
        .filter(|line| matcher.is_match(line))
        .count();

    println!("{}", matches);
}

fn is_part2() -> bool {
    env::args().nth(1).map(|val| val == "2").unwrap_or(false)
}

#[test]
fn example1() {
    let mut rule_set = RuleSet::new();
    let example1 = ["0: 1 2", "1: \"a\"", "2: 1 3 | 3 1", "3: \"b\""];
    for line in example1.iter() {
        let (ix, rule) = rules_grammar::parse_line(line).unwrap();
        rule_set.insert(ix, rule);
    }

    use Rule::*;
    assert_eq!(rule_set.get(&0), Some(&Seq(vec![Ref(1), Ref(2)])));
    assert_eq!(rule_set.get(&1), Some(&Literal('a')));
    assert_eq!(
        rule_set.get(&2),
        Some(&Choice(
            Box::new(Seq(vec![Ref(1), Ref(3)])),
            Box::new(Seq(vec![Ref(3), Ref(1)]))
        ))
    );
    assert_eq!(rule_set.get(&3), Some(&Literal('b')));

    assert_eq!(rule_set_to_regex(&rule_set, false), "^a(ab|ba)$");
}

#[test]
fn example2() {
    let mut rule_set = RuleSet::new();
    let example2 = [
        "0: 4 1 5",
        "1: 2 3 | 3 2",
        "2: 4 4 | 5 5",
        "3: 4 5 | 5 4",
        "4: \"a\"",
        "5: \"b\"",
    ];
    for line in example2.iter() {
        let (ix, rule) = rules_grammar::parse_line(line).unwrap();
        rule_set.insert(ix, rule);
    }

    use Rule::*;
    assert_eq!(rule_set.get(&0), Some(&Seq(vec![Ref(4), Ref(1), Ref(5)])));
    assert_eq!(
        rule_set.get(&1),
        Some(&Choice(
            Box::new(Seq(vec![Ref(2), Ref(3)])),
            Box::new(Seq(vec![Ref(3), Ref(2)]))
        ))
    );
    assert_eq!(
        rule_set.get(&2),
        Some(&Choice(
            Box::new(Seq(vec![Ref(4), Ref(4)])),
            Box::new(Seq(vec![Ref(5), Ref(5)]))
        ))
    );
    assert_eq!(
        rule_set.get(&3),
        Some(&Choice(
            Box::new(Seq(vec![Ref(4), Ref(5)])),
            Box::new(Seq(vec![Ref(5), Ref(4)]))
        ))
    );
    assert_eq!(rule_set.get(&4), Some(&Literal('a')));
    assert_eq!(rule_set.get(&5), Some(&Literal('b')));

    let regex = rule_set_to_regex(&rule_set, false);
    assert_eq!(regex, "^a((aa|bb)(ab|ba)|(ab|ba)(aa|bb))b$");

    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ababbb"));
    assert!(!matcher.is_match("bababa"));
    assert!(matcher.is_match("abbbab"));
    assert!(!matcher.is_match("aaabbb"));
    assert!(!matcher.is_match("aaaabbb"));
}

#[test]
fn example3() {
    let mut rule_set = RuleSet::new();
    let example3 = [
        "42: 9 14 | 10 1",
        "9: 14 27 | 1 26",
        "10: 23 14 | 28 1",
        "1: \"a\"",
        "11: 42 31",
        "5: 1 14 | 15 1",
        "19: 14 1 | 14 14",
        "12: 24 14 | 19 1",
        "16: 15 1 | 14 14",
        "31: 14 17 | 1 13",
        "6: 14 14 | 1 14",
        "2: 1 24 | 14 4",
        "0: 8 11",
        "13: 14 3 | 1 12",
        "15: 1 | 14",
        "17: 14 2 | 1 7",
        "23: 25 1 | 22 14",
        "28: 16 1",
        "4: 1 1",
        "20: 14 14 | 1 15",
        "3: 5 14 | 16 1",
        "27: 1 6 | 14 18",
        "14: \"b\"",
        "21: 14 1 | 1 14",
        "25: 1 1 | 1 14",
        "22: 14 14",
        "8: 42",
        "26: 14 22 | 1 20",
        "18: 15 15",
        "7: 14 5 | 1 21",
        "24: 14 1",
    ];
    for line in example3.iter() {
        let (ix, rule) = rules_grammar::parse_line(line).unwrap();
        rule_set.insert(ix, rule);
    }
    let mut matcher = Regex::new(&rule_set_to_regex(&rule_set, false)).unwrap();

    assert!(!matcher.is_match("abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa"));
    assert!(matcher.is_match("bbabbbbaabaabba"));
    assert!(!matcher.is_match("babbbbaabbbbbabbbbbbaabaaabaaa"));
    assert!(!matcher.is_match("aaabbbbbbaaaabaababaabababbabaaabbababababaaa"));
    assert!(!matcher.is_match("bbbbbbbaaaabbbbaaabbabaaa"));
    assert!(!matcher.is_match("bbbababbbbaaaaaaaabbababaaababaabab"));
    assert!(matcher.is_match("ababaaaaaabaaab"));
    assert!(matcher.is_match("ababaaaaabbbaba"));
    assert!(!matcher.is_match("baabbaaaabbaaaababbaababb"));
    assert!(!matcher.is_match("abbbbabbbbaaaababbbbbbaaaababb"));
    assert!(!matcher.is_match("aaaaabbaabaaaaababaa"));
    assert!(!matcher.is_match("aaaabbaaaabbaaa"));
    assert!(!matcher.is_match("aaaabbaabbaaaaaaabbbabbbaaabbaabaaa"));
    assert!(!matcher.is_match("babaaabbbaaabaababbaabababaaab"));
    assert!(!matcher.is_match("aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"));

    matcher = Regex::new(&rule_set_to_regex(&rule_set, true)).unwrap();

    assert!(!matcher.is_match("abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa"));
    assert!(matcher.is_match("bbabbbbaabaabba"));
    assert!(matcher.is_match("babbbbaabbbbbabbbbbbaabaaabaaa"));
    assert!(matcher.is_match("aaabbbbbbaaaabaababaabababbabaaabbababababaaa"));
    assert!(matcher.is_match("bbbbbbbaaaabbbbaaabbabaaa"));
    assert!(matcher.is_match("bbbababbbbaaaaaaaabbababaaababaabab"));
    assert!(matcher.is_match("ababaaaaaabaaab"));
    assert!(matcher.is_match("ababaaaaabbbaba"));
    assert!(matcher.is_match("baabbaaaabbaaaababbaababb"));
    assert!(matcher.is_match("abbbbabbbbaaaababbbbbbaaaababb"));
    assert!(matcher.is_match("aaaaabbaabaaaaababaa"));
    assert!(!matcher.is_match("aaaabbaaaabbaaa"));
    assert!(matcher.is_match("aaaabbaabbaaaaaaabbbabbbaaabbaabaaa"));
    assert!(!matcher.is_match("babaaabbbaaabaababbaabababaaab"));
    assert!(matcher.is_match("aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"));
}
