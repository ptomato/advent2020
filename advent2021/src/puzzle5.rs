use ndarray::prelude::*;
use std::cmp::{max, min};

type Pair = ((usize, usize), (usize, usize));

fn read_input(input: &str) -> impl Iterator<Item = Pair> + '_ {
    input.lines().map(|line| {
        let (x1, y1, x2, y2) =
            scan_fmt!(line, "{},{} -> {},{}", usize, usize, usize, usize).unwrap();
        ((x1, y1), (x2, y2))
    })
}

fn draw_line(grid: &mut Array2<usize>, ((x1, y1), (x2, y2)): Pair) {
    let increment = if x1 == x2 || y1 == y2 {
        array![[1]] // Horizontal or vertical line
    } else if (x1 > x2 && y1 > y2) || (x1 < x2 && y1 < y2) {
        Array::eye(max(x1, x2) - min(x1, x2) + 1) // NW-SE diagonal
    } else {
        // SW-NE diagonal
        Array::eye(max(x1, x2) - min(x1, x2) + 1)
            .slice(s![.., ..;-1])
            .into_owned()
    };
    let (x1, x2) = (min(x1, x2), max(x1, x2));
    let (y1, y2) = (min(y1, y2), max(y1, y2));
    let mut slice = grid.slice_mut(s![x1..=x2, y1..=y2]);
    slice += &increment;
}

fn count_dangerous_areas(grid: &Array2<usize>) -> usize {
    grid.mapv(|x| if x > 1 { 1 } else { 0 }).sum()
}

pub fn main(is_part2: bool) {
    let input = include_str!("input/puzzle5");
    let mut grid = Array::zeros((1000, 1000));
    read_input(input)
        .filter(|((x1, y1), (x2, y2))| if is_part2 { true } else { x1 == x2 || y1 == y2 })
        .for_each(|coords| draw_line(&mut grid, coords));
    println!("{}", count_dangerous_areas(&grid));
}

#[cfg(test)]
static EXAMPLE_COORDS: [Pair; 10] = [
    ((0, 9), (5, 9)),
    ((8, 0), (0, 8)),
    ((9, 4), (3, 4)),
    ((2, 2), (2, 1)),
    ((7, 0), (7, 4)),
    ((6, 4), (2, 0)),
    ((0, 9), (2, 9)),
    ((3, 4), (1, 4)),
    ((0, 0), (8, 8)),
    ((5, 5), (8, 2)),
];

#[test]
fn parse_input() {
    static INPUT: &str = "\
        0,9 -> 5,9\n\
        8,0 -> 0,8\n\
        9,4 -> 3,4\n\
        2,2 -> 2,1\n\
        7,0 -> 7,4\n\
        6,4 -> 2,0\n\
        0,9 -> 2,9\n\
        3,4 -> 1,4\n\
        0,0 -> 8,8\n\
        5,5 -> 8,2\n";
    assert_eq!(read_input(INPUT).collect::<Vec<Pair>>(), &EXAMPLE_COORDS);
}

#[test]
fn part1_example() {
    let mut grid = Array::zeros((10, 10));
    EXAMPLE_COORDS
        .iter()
        .filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2)
        .for_each(|&coords| draw_line(&mut grid, coords));
    assert_eq!(
        grid,
        array![
            [0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0, 0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0, 0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            [0, 1, 1, 2, 1, 1, 1, 2, 1, 1],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [2, 2, 2, 1, 1, 1, 0, 0, 0, 0],
        ]
        .t()
    );
    assert_eq!(count_dangerous_areas(&grid), 5);
}

#[test]
fn part2_example() {
    let mut grid = Array::zeros((10, 10));
    EXAMPLE_COORDS
        .iter()
        .for_each(|&coords| draw_line(&mut grid, coords));
    assert_eq!(
        grid,
        array![
            [1, 0, 1, 0, 0, 0, 0, 1, 1, 0],
            [0, 1, 1, 1, 0, 0, 0, 2, 0, 0],
            [0, 0, 2, 0, 1, 0, 1, 1, 1, 0],
            [0, 0, 0, 1, 0, 2, 0, 2, 0, 0],
            [0, 1, 1, 2, 3, 1, 3, 2, 1, 1],
            [0, 0, 0, 1, 0, 2, 0, 0, 0, 0],
            [0, 0, 1, 0, 0, 0, 1, 0, 0, 0],
            [0, 1, 0, 0, 0, 0, 0, 1, 0, 0],
            [1, 0, 0, 0, 0, 0, 0, 0, 1, 0],
            [2, 2, 2, 1, 1, 1, 0, 0, 0, 0],
        ]
        .t()
    );
    assert_eq!(count_dangerous_areas(&grid), 12);
}
