use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq)]
struct Board {
    rows: [HashSet<u8>; 10],
}

impl Board {
    fn from_grid(grid: &[[u8; 5]; 5]) -> Board {
        let mut rows: [HashSet<u8>; 10] = Default::default();
        for i in 0..5 {
            rows[i].extend(grid[i]);
            for j in 0..5 {
                rows[j + 5].insert(grid[i][j]);
            }
        }
        Board { rows }
    }

    fn from_string(s: &str) -> Board {
        let mut grid = [[0; 5]; 5];
        for (ix, line) in s.lines().enumerate() {
            for jx in 0..5 {
                grid[ix][jx] = line[(jx * 3)..(jx * 3 + 2)].trim().parse().unwrap();
            }
        }
        Board::from_grid(&grid)
    }

    fn mark(&mut self, num: u8) -> bool {
        let mut won = false;
        for ix in 0..10 {
            self.rows[ix].remove(&num);
            won = won || self.rows[ix].is_empty();
        }
        won
    }

    fn has_won(&self) -> bool {
        for ix in 0..10 {
            if self.rows[ix].is_empty() {
                return true;
            }
        }
        false
    }

    fn sum_unmarked(&self) -> u32 {
        let mut all_numbers: HashSet<u32> = HashSet::new();
        for ix in 0..5 {
            all_numbers.extend(self.rows[ix].iter().map(|&v| v as u32));
        }
        all_numbers.iter().sum()
    }
}

pub fn main(is_part2: bool) {
    let input = include_str!("input/puzzle4");
    let mut parts = input.split("\n\n");
    let draws = parts
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u8>().unwrap());
    let mut boards: Vec<_> = parts.map(Board::from_string).collect();
    let mut score_of_last_winning = None;
    for draw in draws {
        for board in &mut boards {
            let already_won = board.has_won();
            if board.mark(draw) {
                if is_part2 {
                    if !already_won {
                        score_of_last_winning = Some(board.sum_unmarked() * draw as u32);
                    }
                } else {
                    println!("{}", board.sum_unmarked() * draw as u32);
                    return;
                }
            }
        }
        if is_part2 && boards.iter().all(Board::has_won) {
            println!("{}", score_of_last_winning.unwrap());
            return;
        }
    }
    panic!("no board won");
}

#[cfg(test)]
static EXAMPLE_GRIDS: [[[u8; 5]; 5]; 3] = [
    [
        [22, 13, 17, 11, 0],
        [8, 2, 23, 4, 24],
        [21, 9, 14, 16, 7],
        [6, 10, 3, 18, 5],
        [1, 12, 20, 15, 19],
    ],
    [
        [3, 15, 0, 2, 22],
        [9, 18, 13, 17, 5],
        [19, 8, 7, 25, 23],
        [20, 11, 10, 24, 4],
        [14, 21, 16, 12, 6],
    ],
    [
        [14, 21, 17, 24, 4],
        [10, 16, 15, 9, 19],
        [18, 8, 23, 26, 20],
        [22, 11, 13, 6, 5],
        [2, 0, 12, 3, 7],
    ],
];

#[test]
fn board_from_grid() {
    let board = Board::from_grid(&EXAMPLE_GRIDS[0]);
    assert_eq!(
        board.rows,
        [
            [22, 13, 17, 11, 0].into(),
            [8, 2, 23, 4, 24].into(),
            [21, 9, 14, 16, 7].into(),
            [6, 10, 3, 18, 5].into(),
            [1, 12, 20, 15, 19].into(),
            [22, 8, 21, 6, 1].into(),
            [13, 2, 9, 10, 12].into(),
            [17, 23, 14, 3, 20].into(),
            [11, 4, 16, 18, 15].into(),
            [0, 24, 7, 5, 19].into(),
        ]
    );
}

#[test]
fn board_from_string() {
    assert_eq!(
        Board::from_string(
            "\
22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19\
    "
        ),
        Board::from_grid(&EXAMPLE_GRIDS[0])
    );
}

#[test]
fn part1_example() {
    let mut boards = [
        Board::from_grid(&EXAMPLE_GRIDS[0]),
        Board::from_grid(&EXAMPLE_GRIDS[1]),
        Board::from_grid(&EXAMPLE_GRIDS[2]),
    ];
    for draw in [7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21] {
        for ix in 0..3 {
            assert!(!boards[ix].mark(draw));
        }
    }
    assert!(!boards[0].mark(24));
    assert!(!boards[1].mark(24));
    assert!(boards[2].mark(24));
    assert_eq!(boards[2].sum_unmarked(), 188);
}

#[test]
fn part2_example() {
    let mut boards = [
        Board::from_grid(&EXAMPLE_GRIDS[0]),
        Board::from_grid(&EXAMPLE_GRIDS[1]),
        Board::from_grid(&EXAMPLE_GRIDS[2]),
    ];
    for draw in [7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16] {
        boards[0].mark(draw);
        boards[2].mark(draw);
        assert!(!boards[1].mark(draw));
    }
    assert!(boards[0].has_won());
    assert!(boards[2].has_won());
    assert!(boards[1].mark(13));
    assert_eq!(boards[1].sum_unmarked(), 148);
}
