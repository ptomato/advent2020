use itertools::Itertools;
use std::env;

fn dec_nonnegative_mod(num: usize, n_cups: usize) -> usize {
    (num + n_cups - 2) % n_cups + 1
}

struct Links {
    n_cups: usize,
    current_cup: usize,
    links: Vec<usize>,
}

impl Links {
    fn from_list(cups: &[usize]) -> Self {
        let n_cups = cups.len();
        let mut links = vec![0; n_cups + 1];
        for (&this, &next) in cups.iter().tuple_windows() {
            links[this] = next;
        }
        links[cups[n_cups - 1]] = cups[0];
        Self {
            n_cups,
            current_cup: cups[0],
            links,
        }
    }

    #[cfg(test)]
    fn to_list(&self) -> Vec<usize> {
        let mut retval = vec![0; self.n_cups];
        retval[0] = self.current_cup;
        for ix in 1..self.n_cups {
            retval[ix] = self.links[retval[ix - 1]];
        }
        retval
    }

    fn next(&self, cup: usize) -> usize {
        self.links[cup]
    }

    fn do_move(&mut self) {
        let move1 = self.links[self.current_cup];
        let move2 = self.links[move1];
        let move3 = self.links[move2];
        let mut insert_after = dec_nonnegative_mod(self.current_cup, self.n_cups);
        while insert_after == move1 || insert_after == move2 || insert_after == move3 {
            insert_after = dec_nonnegative_mod(insert_after, self.n_cups);
        }
        let next_current_cup = self.links[move3];
        self.links[self.current_cup] = next_current_cup;
        let insert_before = self.links[insert_after];
        self.links[insert_after] = move1;
        self.links[move3] = insert_before;
        self.current_cup = next_current_cup;
    }
}

fn main() {
    let input = "253149867";
    let n_cups = if is_part2() { 1_000_000 } else { 9 };
    let cups: Vec<usize> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .chain(10..(n_cups + 1))
        .collect();
    let mut links = Links::from_list(&cups);
    let n_moves = if is_part2() { 10_000_000 } else { 100 };
    let progress = indicatif::ProgressBar::new(n_moves);
    progress.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("[{eta_precise} left] {wide_bar} {pos}/{len}"),
    );
    for _ in 0..n_moves {
        links.do_move();
        progress.inc(1);
    }
    progress.finish_and_clear();
    if is_part2() {
        let next = links.next(1);
        let next2 = links.next(next);
        println!("{}", next * next2);
    } else {
        let mut cup = links.next(1);
        let mut order = vec![];
        while cup != 1 {
            order.push(cup.to_string());
            cup = links.next(cup);
        }
        println!("{}", order.join(""));
    }
}

fn is_part2() -> bool {
    env::args().nth(1).map(|val| val == "2").unwrap_or(false)
}

#[test]
fn example_part1() {
    let cups = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
    let mut links = Links::from_list(&cups);
    assert_eq!(links.to_list(), [3, 8, 9, 1, 2, 5, 4, 6, 7]);
    links.do_move();
    assert_eq!(links.to_list(), [2, 8, 9, 1, 5, 4, 6, 7, 3]);
    links.do_move();
    assert_eq!(links.to_list(), [5, 4, 6, 7, 8, 9, 1, 3, 2]);
    links.do_move();
    assert_eq!(links.to_list(), [8, 9, 1, 3, 4, 6, 7, 2, 5]);
    links.do_move();
    assert_eq!(links.to_list(), [4, 6, 7, 9, 1, 3, 2, 5, 8]);
    links.do_move();
    assert_eq!(links.to_list(), [1, 3, 6, 7, 9, 2, 5, 8, 4]);
    links.do_move();
    assert_eq!(links.to_list(), [9, 3, 6, 7, 2, 5, 8, 4, 1]);
    links.do_move();
    assert_eq!(links.to_list(), [2, 5, 8, 3, 6, 7, 4, 1, 9]);
    links.do_move();
    assert_eq!(links.to_list(), [6, 7, 4, 1, 5, 8, 3, 9, 2]);
    links.do_move();
    assert_eq!(links.to_list(), [5, 7, 4, 1, 8, 3, 9, 2, 6]);
    links.do_move();
    assert_eq!(links.to_list(), [8, 3, 7, 4, 1, 9, 2, 6, 5]);
}
