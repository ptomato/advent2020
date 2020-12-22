use std::collections::{HashSet, VecDeque};
use std::env;

type Deck = VecDeque<usize>;

fn main() {
    let input = include_str!("input");
    let mut deck_blocks = input.split("\n\n");
    let mut deck1 = read_deck(deck_blocks.next().unwrap());
    let mut deck2 = read_deck(deck_blocks.next().unwrap());
    if is_part2() {
        play_recursive_combat(&mut deck1, &mut deck2);
    } else {
        play_combat(&mut deck1, &mut deck2);
    }
    println!(
        "{}",
        score_deck(if deck1.is_empty() { &deck2 } else { &deck1 })
    );
}

fn read_deck(block: &str) -> Deck {
    block.lines().skip(1).map(|s| s.parse().unwrap()).collect()
}

fn play_combat(deck1: &mut Deck, deck2: &mut Deck) {
    while !deck1.is_empty() && !deck2.is_empty() {
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();
        if card1 > card2 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }
}

fn play_recursive_combat(deck1: &mut Deck, deck2: &mut Deck) -> bool {
    let mut played_rounds = HashSet::new();
    while !deck1.is_empty() && !deck2.is_empty() {
        let this_round = (deck1.clone(), deck2.clone());
        if played_rounds.contains(&this_round) {
            return true;
        }
        played_rounds.insert(this_round);
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();
        let player1_wins = if deck1.len() >= card1 && deck2.len() >= card2 {
            let mut deck1_copy = deck1.clone();
            let mut deck2_copy = deck2.clone();
            deck1_copy.truncate(card1);
            deck2_copy.truncate(card2);
            play_recursive_combat(&mut deck1_copy, &mut deck2_copy)
        } else {
            card1 > card2
        };

        if player1_wins {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }
    deck2.is_empty()
}

fn score_deck(deck: &Deck) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(ix, val)| (ix + 1) * val)
        .sum()
}

fn is_part2() -> bool {
    env::args().nth(1).map(|val| val == "2").unwrap_or(false)
}

#[test]
fn example_part1() {
    let mut deck1 = VecDeque::from(vec![9, 2, 6, 3, 1]);
    let mut deck2 = VecDeque::from(vec![5, 8, 4, 7, 10]);
    play_combat(&mut deck1, &mut deck2);
    assert_eq!(score_deck(&deck1), 0);
    assert_eq!(score_deck(&deck2), 306);
}

#[test]
fn example_part2() {
    let mut deck1 = VecDeque::from(vec![9, 2, 6, 3, 1]);
    let mut deck2 = VecDeque::from(vec![5, 8, 4, 7, 10]);
    play_recursive_combat(&mut deck1, &mut deck2);
    assert_eq!(score_deck(&deck1), 0);
    assert_eq!(score_deck(&deck2), 291);
}

#[test]
fn example_infinite() {
    let mut deck1 = VecDeque::from(vec![43, 19]);
    let mut deck2 = VecDeque::from(vec![2, 29, 14]);
    assert!(play_recursive_combat(&mut deck1, &mut deck2));
}
