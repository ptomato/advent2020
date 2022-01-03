use ndarray::prelude::*;

fn parse(s: &str) -> Array2<u8> {
    let lines: Vec<&str> = s.lines().collect();
    let mut result = Array2::zeros((lines.len(), lines[0].len()));
    for (i, line) in lines.iter().enumerate() {
        for (j, b) in line.bytes().enumerate() {
            result[(i, j)] = match b {
                b'0' => 0,
                b'1' => 1,
                b'2' => 2,
                b'3' => 3,
                b'4' => 4,
                b'5' => 5,
                b'6' => 6,
                b'7' => 7,
                b'8' => 8,
                b'9' => 9,
                _ => panic!("Unexpected input {}", b),
            }
        }
    }
    result
}

fn low_points(arr: &Array2<u8>) -> Vec<(usize, usize)> {
    let blank = Array2::from_elem(arr.raw_dim(), 9);
    let mut shift_left = blank.clone();
    shift_left
        .slice_mut(s![..-1, ..])
        .assign(&arr.slice(s![1.., ..]));
    let mut shift_right = blank.clone();
    shift_right
        .slice_mut(s![1.., ..])
        .assign(&arr.slice(s![..-1, ..]));
    let mut shift_up = blank.clone();
    shift_up
        .slice_mut(s![.., ..-1])
        .assign(&arr.slice(s![.., 1..]));
    let mut shift_down = blank.clone();
    shift_down
        .slice_mut(s![.., 1..])
        .assign(&arr.slice(s![.., ..-1]));
    let mut result = Array2::zeros(arr.raw_dim());
    azip!((res in &mut result, &h in arr, &l in &shift_left, &r in &shift_right, &u in &shift_up, &d in &shift_down)
            *res = if h < l && h < r && h < u && h < d {1} else {0});
    result
        .indexed_iter()
        .filter_map(|(index, &v)| if v != 0 { Some(index) } else { None })
        .collect()
}

fn risk_levels(arr: &Array2<u8>) -> Vec<usize> {
    low_points(&arr)
        .iter()
        .map(|&index| (arr[index] + 1) as usize)
        .collect()
}

fn flood_fill(basin: &mut Array2<usize>, arr: &Array2<u8>, (row, col): (usize, usize)) {
    if arr[(row, col)] == 9 || basin[(row, col)] == 1 {
        return;
    }
    basin[(row, col)] = 1;
    if row != arr.nrows() - 1 {
        flood_fill(basin, arr, (row + 1, col));
    }
    if row != 0 {
        flood_fill(basin, arr, (row - 1, col));
    }
    if col != 0 {
        flood_fill(basin, arr, (row, col - 1));
    }
    if col != arr.ncols() - 1 {
        flood_fill(basin, arr, (row, col + 1));
    }
}

fn basins(arr: &Array2<u8>) -> Vec<usize> {
    low_points(&arr)
        .iter()
        .map(|&index| {
            let mut basin = Array2::<usize>::zeros(arr.raw_dim());
            flood_fill(&mut basin, arr, index);
            basin.sum()
        })
        .collect()
}

pub fn main() {
    let grid = parse(include_str!("input/puzzle9"));
    println!("{}", risk_levels(&grid).iter().sum::<usize>());
    let mut basins = basins(&grid);
    basins.sort();
    println!("{}", basins.iter().rev().take(3).product::<usize>());
}

#[cfg(test)]
fn gen_example() -> Array2<u8> {
    array![
        [2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
        [3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
        [9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
        [8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
        [9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
    ]
}

#[test]
fn test_parse() {
    assert_eq!(
        parse(
            "\
2199943210
3987894921
9856789892
8767896789
9899965678
"
        ),
        gen_example()
    );
}

#[test]
fn equal_height() {
    let grid = array![
        [0, 1, 1, 1, 0],
        [1, 1, 9, 1, 1],
        [1, 9, 9, 9, 1],
        [1, 1, 9, 1, 1],
        [0, 1, 1, 1, 0],
    ];
    assert_eq!(low_points(&grid), [(0, 0), (0, 4), (4, 0), (4, 4)]);
}

#[test]
fn test_flood_fill() {
    let example = gen_example();
    let mut basin = Array2::<usize>::zeros(example.raw_dim());
    flood_fill(&mut basin, &example, (0, 9));
    assert_eq!(
        basin,
        array![
            [0, 0, 0, 0, 0, 1, 1, 1, 1, 1],
            [0, 0, 0, 0, 0, 0, 1, 0, 1, 1],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ]
    );
}

#[test]
fn part1_example() {
    let example = gen_example();
    assert_eq!(low_points(&example), [(0, 1), (0, 9), (2, 2), (4, 6)]);
    assert_eq!(risk_levels(&example).iter().sum::<usize>(), 15);
}

#[test]
fn part2_example() {
    let example = gen_example();
    let mut basins = basins(&example);
    assert_eq!(basins, [3, 9, 14, 9]);
    basins.sort();
    assert_eq!(basins.iter().rev().take(3).product::<usize>(), 1134);
}
