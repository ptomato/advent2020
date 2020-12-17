use itertools::Itertools;
use ndarray::{s, Array4};
use std::iter;

fn main() {
    let input = include_str!("input");
    let n_turns = 6;
    let mut grid = read_grid(input, n_turns);

    for _ in 0..n_turns {
        let neighbours = calc_neighbours(&grid);
        let activations = &neighbours.mapv(|count| (count == 3) as i16)
            * &grid.mapv(|active| (active == 0) as i16);
        let deactivations = &neighbours.mapv(|count| (count < 2 || count > 3) as i16) * &grid;
        grid = grid + activations - deactivations;
    }

    println!("{}", grid.sum());
}

fn calc_neighbours(grid: &Array4<i16>) -> Array4<i16> {
    let shape = grid.shape();
    let width = shape[0] as isize;
    let height = shape[1] as isize;
    let depth = shape[2] as isize;
    let limit4 = shape[3] as isize;
    let mut neighbours = Array4::<i16>::zeros(grid.raw_dim());
    // Add slices of the occupied grid shifted one space in each direction
    for starts in iter::repeat(-1..=1).take(4).multi_cartesian_product() {
        if starts.iter().all(|start| *start == 0) {
            continue;
        }
        let (xstart, ystart, zstart, wstart) = (starts[0], starts[1], starts[2], starts[3]);
        let xdest = xstart.max(0)..(width + xstart).min(width);
        let ydest = ystart.max(0)..(height + ystart).min(height);
        let zdest = zstart.max(0)..(depth + zstart).min(depth);
        let wdest = wstart.max(0)..(limit4 + wstart).min(limit4);
        let xsource = (-xstart).max(0)..(width - xstart).min(width);
        let ysource = (-ystart).max(0)..(height - ystart).min(height);
        let zsource = (-zstart).max(0)..(depth - zstart).min(depth);
        let wsource = (-wstart).max(0)..(limit4 - wstart).min(limit4);
        let mut slice = neighbours.slice_mut(s![xdest, ydest, zdest, wdest]);
        slice += &grid.slice(s![xsource, ysource, zsource, wsource]);
    }
    neighbours
}

fn read_grid(input: &str, padding: usize) -> Array4<i16> {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines[0].len();
    let mut cells = Array4::zeros((
        width + 2 * padding,
        height + 2 * padding,
        2 * padding + 1,
        2 * padding + 1,
    ));
    for (y, line) in lines.iter().enumerate() {
        for (x, tile) in line.bytes().enumerate() {
            cells[[x + padding, y + padding, padding, padding]] = match tile {
                b'#' => 1,
                b'.' => 0,
                _ => panic!("Bad tile '{}'", tile),
            };
        }
    }
    cells
}
