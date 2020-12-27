#[macro_use]
extern crate scan_fmt;

use bit_reverse::ParallelReverse;
use itertools::Itertools;
use multimap::MultiMap;
use ndarray::{concatenate, s, Array2, ArrayView, ArrayView2, Axis, Ix1};
use std::collections::HashSet;
use std::convert::TryInto;
use std::env;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    TOP = 0,
    RIGHT = 1,
    BOTTOM = 2,
    LEFT = 3,
}

impl Direction {
    fn opposite(&self) -> Self {
        use Direction::*;
        match *self {
            TOP => BOTTOM,
            RIGHT => LEFT,
            BOTTOM => TOP,
            LEFT => RIGHT,
        }
    }

    // returns the number of times to call rot90() on @other, to make it point
    // the same way as @self
    fn difference(self, other: Self) -> usize {
        ((4 + (other as i8) - (self as i8)) % 4) as usize
    }

    fn all() -> [Direction; 4] {
        use Direction::*;
        [TOP, RIGHT, BOTTOM, LEFT]
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Tile {
    id: u64,
    // top, right, bottom, left borders with bits counted from LSB to MSB in
    // clockwise direction
    borders: [u16; 4],
    border_bits: u8, // number of bits in border
    image: Array2<u8>,
}

impl Tile {
    fn new(id: u64, grid: &Array2<u8>) -> Self {
        let borders = [
            s![0, ..],
            s![.., grid.ncols() - 1],
            s![grid.nrows() - 1, ..;-1],
            s![..;-1, 0],
        ]
        .iter()
        .map(|&slice| to_bits(grid.slice(slice)))
        .collect::<Vec<u16>>()
        .try_into()
        .unwrap();
        let image = grid
            .slice(s![1..grid.nrows() - 1, 1..grid.ncols() - 1])
            .into_owned();
        Tile {
            id,
            borders,
            image,
            border_bits: grid.ncols() as u8,
        }
    }

    // rotate counterclockwise n times
    fn rot90(&self, n: usize) -> Self {
        let rotated_view = match n % 4 {
            0 => self.image.view(),
            1 => self.image.slice(s![.., ..;-1]).reversed_axes(),
            2 => self.image.slice(s![..;-1, ..;-1]),
            3 => self.image.slice(s![..;-1, ..]).reversed_axes(),
            _ => panic!("Impossible"),
        };
        let rotated_image = rotated_view.into_owned();
        let mut borders = self.borders;
        borders.rotate_left(n);
        Tile {
            id: self.id,
            borders,
            image: rotated_image,
            border_bits: self.border_bits,
        }
    }

    fn fliplr(&self) -> Self {
        let flipped_image = self.image.slice(s![.., ..;-1]).into_owned();
        let mut borders: Vec<_> = self
            .borders
            .iter()
            .map(|&b| flip_bits(b, self.border_bits))
            .collect();
        borders.reverse();
        borders.rotate_right(1);
        Tile {
            id: self.id,
            borders: borders.try_into().unwrap(),
            image: flipped_image,
            border_bits: self.border_bits,
        }
    }

    fn connection_side(&self, other: &Self) -> Option<Direction> {
        let mut borders2: HashSet<_> = other.borders.iter().cloned().collect();
        borders2.extend(
            other
                .borders
                .iter()
                .map(|&b| flip_bits(b, self.border_bits)),
        );
        for (&border, &direction) in self.borders.iter().zip(Direction::all().iter()) {
            if borders2.contains(&border) {
                return Some(direction);
            }
        }
        None
    }

    fn connects_in_direction(&self, direction: Direction, other: &Self) -> Option<Direction> {
        let border_to_connect = self.borders[direction as usize];
        other
            .borders
            .iter()
            .zip(Direction::all().iter())
            .find(|(&border, _)| border == flip_bits(border_to_connect, self.border_bits))
            .map(|(_, &other_side)| other_side)
    }

    fn match_other(&self, other: &Tile, direction: Direction) -> Option<Tile> {
        match self.connects_in_direction(direction, other) {
            None => (),
            Some(dir) => {
                let rotations = direction.opposite().difference(dir);
                return Some(other.clone().rot90(rotations));
            }
        }
        let flipped = other.fliplr();
        match self.connects_in_direction(direction, &flipped) {
            None => (),
            Some(dir) => {
                let rotations = direction.opposite().difference(dir);
                return Some(flipped.rot90(rotations));
            }
        }
        None
    }
}

fn flip_bits(border: u16, n_bits: u8) -> u16 {
    assert!(n_bits <= 16);
    border.swap_bits() >> (16 - n_bits)
}

fn to_bits(slice: ArrayView<u8, Ix1>) -> u16 {
    let mut retval = 0;
    for (ix, &cell) in slice.iter().enumerate() {
        if cell > 0 {
            retval |= 2_u16.pow(ix.try_into().unwrap());
        }
    }
    retval
}

fn main() {
    let input = include_str!("input");
    let tiles = read_input(input);
    let mut solver = Solver::new(&tiles);

    if is_part2() {
        let full_image = solver.arrange();
        let (_, pixels) = all_orientations(&full_image)
            .iter()
            .find_map(|image| {
                let (count, pixels) = count_sea_monsters(image);
                if count != 0 {
                    Some((count, pixels))
                } else {
                    None
                }
            })
            .unwrap();
        println!("{}", full_image.iter().filter(|&&c| c > 0).count() - pixels);
    } else {
        let answer: u64 = solver.corners.iter().product();
        println!("{}", answer);
    }
}

struct Solver {
    tiles: Vec<Tile>,
    connections: MultiMap<u64, u64>,
    corners: [u64; 4],
    used_tile_ids: HashSet<u64>,
}

impl Solver {
    fn new(tiles: &[Tile]) -> Self {
        let mut connections = MultiMap::new();
        for (tile1, tile2) in tiles.iter().tuple_combinations() {
            if tile1.connection_side(tile2).is_some() {
                connections.insert(tile1.id, tile2.id);
                connections.insert(tile2.id, tile1.id);
            }
        }
        let corners: Vec<_> = tiles
            .iter()
            .map(|tile| tile.id)
            .filter(|id| match connections.get_vec(id).unwrap().len() {
                2 => true,
                3 | 4 => false,
                _ => panic!("Impossible"),
            })
            .collect();
        Self {
            tiles: tiles.to_vec(),
            connections,
            corners: corners.try_into().unwrap(),
            used_tile_ids: HashSet::new(),
        }
    }

    fn find_and_orient_tile(&mut self, tile: &Tile, direction: Direction) -> Option<Tile> {
        let tile_connections = self.connections.get_vec(&tile.id).unwrap();
        let maybe_next_tile = self
            .tiles
            .iter()
            .filter(|t| tile_connections.contains(&t.id) && !self.used_tile_ids.contains(&t.id))
            .find_map(|candidate| tile.match_other(candidate, direction));
        if let Some(t) = &maybe_next_tile {
            self.used_tile_ids.insert(t.id);
        }
        maybe_next_tile
    }

    fn arrange(&mut self) -> Array2<u8> {
        // Find top left corner - pick an arbitrary corner tile and rotate it until
        // it has connections on the right and bottom
        let mut tl_corner = self
            .tiles
            .iter()
            .find(|tile| self.corners.contains(&tile.id))
            .unwrap()
            .clone();
        self.used_tile_ids.insert(tl_corner.id);
        let tl_corner_connections: Vec<_> = self
            .tiles
            .iter()
            .filter(|t| {
                self.connections
                    .get_vec(&tl_corner.id)
                    .unwrap()
                    .contains(&t.id)
            })
            .map(|candidate| tl_corner.connection_side(&candidate))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect();
        assert_eq!(tl_corner_connections.len(), 2);
        tl_corner = tl_corner.rot90(match (tl_corner_connections[0], tl_corner_connections[1]) {
            (Direction::RIGHT, Direction::BOTTOM) | (Direction::BOTTOM, Direction::RIGHT) => 0,
            (Direction::LEFT, Direction::BOTTOM) | (Direction::BOTTOM, Direction::LEFT) => 1,
            (Direction::LEFT, Direction::TOP) | (Direction::TOP, Direction::LEFT) => 2,
            (Direction::RIGHT, Direction::TOP) | (Direction::TOP, Direction::RIGHT) => 3,
            _ => panic!("Impossible {:?}", tl_corner_connections),
        });

        // Build the top edge
        let mut t_row = vec![tl_corner];
        loop {
            match self.find_and_orient_tile(&&t_row[t_row.len() - 1], Direction::RIGHT) {
                None => break,
                Some(tile) => {
                    t_row.push(tile);
                }
            }
        }

        let ncols = t_row.len();
        let nrows = self.tiles.len() / ncols;

        println!("whole image is {}×{}", ncols, nrows);

        // For each subsequent row...
        let mut rows = vec![t_row];
        for row in 1..nrows {
            // Arrange the tiles that connect to the ones in the row above
            rows.push(
                (0..ncols)
                    .map(|col| {
                        self.find_and_orient_tile(&rows[row - 1][col], Direction::BOTTOM)
                            .unwrap()
                    })
                    .collect(),
            );
        }

        // Concatenate all the image data together
        let all_rows: Vec<_> = rows
            .iter()
            .map(|row| {
                let row_images: Vec<_> = row.iter().map(|t| t.image.view()).collect();
                concatenate(Axis(1), &row_images).unwrap()
            })
            .collect();
        concatenate(
            Axis(0),
            &all_rows.iter().map(|row| row.view()).collect::<Vec<_>>(),
        )
        .unwrap()
    }
}

fn all_orientations(image: &Array2<u8>) -> [ArrayView2<u8>; 8] {
    [
        image.view(),
        image.view().reversed_axes(),
        image.slice(s![.., ..;-1]),
        image.slice(s![.., ..;-1]).reversed_axes(),
        image.slice(s![..;-1, ..]),
        image.slice(s![..;-1, ..]).reversed_axes(),
        image.slice(s![..;-1, ..;-1]),
        image.slice(s![..;-1, ..;-1]).reversed_axes(),
    ]
}

static SEA_MONSTER: [&str; 3] = [
    "                  # ",
    "#    ##    ##    ###",
    " #  #  #  #  #  #   ",
];

fn count_sea_monsters(image: &ArrayView2<u8>) -> (usize, usize) {
    let mon_rows = SEA_MONSTER.len();
    let mon_cols = SEA_MONSTER[0].len();
    let mut sea_monster = Array2::zeros((mon_rows, mon_cols));
    for (y, line) in SEA_MONSTER.iter().enumerate() {
        for (x, cell) in line.bytes().enumerate() {
            sea_monster[[y, x]] = (cell != b' ') as u8;
        }
    }
    let mon_pixels: u8 = sea_monster.iter().sum();

    let mut monsters = 0;
    let rows = image.nrows();
    let cols = image.ncols();
    for y in 0..(rows - mon_rows) {
        for x in 0..(cols - mon_cols) {
            let slice = image.slice(s![y..(y + mon_rows), x..(x + mon_cols)]);
            let correlation = &slice * &sea_monster.view();
            if correlation.iter().sum::<u8>() == mon_pixels {
                monsters += 1;
            }
        }
    }
    (monsters, monsters * mon_pixels as usize)
}

fn read_grid(lines: &[&str]) -> Array2<u8> {
    let rows = lines.len();
    let cols = lines[0].len();
    let mut cells = Array2::zeros((rows, cols));
    for (y, line) in lines.iter().enumerate() {
        for (x, tile) in line.bytes().enumerate() {
            cells[[y, x]] = match tile {
                b'#' => 1,
                b'.' => 0,
                _ => panic!("Bad tile '{}'", tile),
            };
        }
    }
    cells
}

fn read_tile(input: &str) -> Tile {
    let mut lines = input.lines();
    let header = lines.next().unwrap();
    let id = scan_fmt!(header, "Tile {}:", u64).unwrap();
    let grid = read_grid(&lines.collect::<Vec<&str>>());
    Tile::new(id, &grid)
}

fn read_input(input: &'static str) -> Vec<Tile> {
    input
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(read_tile)
        .collect()
}

fn is_part2() -> bool {
    env::args().nth(1).map(|val| val == "2").unwrap_or(false)
}

#[test]
fn example() {
    let input = include_str!("test_input");
    let tiles = read_input(input);

    assert_eq!(
        tiles.iter().map(|t| t.id).collect::<Vec<u64>>(),
        [2311, 1951, 1171, 1427, 1489, 2473, 2971, 2729, 3079]
    );

    let mut solver = Solver::new(&tiles);
    assert_eq!(
        tiles
            .iter()
            .map(|t| solver.connections.get_vec(&t.id).unwrap().len())
            .collect::<Vec<usize>>(),
        [3, 2, 2, 4, 3, 3, 2, 3, 2]
    );

    assert_eq!(solver.corners.len(), 4);

    let full_image = solver.arrange();
    let monsters = all_orientations(&full_image).iter().find_map(|image| {
        let (count, pixels) = count_sea_monsters(image);
        if count != 0 {
            Some((count, pixels))
        } else {
            None
        }
    });
    let (count, pixels) = monsters.unwrap();
    assert_eq!(count, 2);
    assert_eq!(pixels, 30);
    assert_eq!(full_image.iter().filter(|&&c| c > 0).count() - pixels, 273);
}

#[test]
fn direction_opposite() {
    assert_eq!(Direction::LEFT.opposite(), Direction::RIGHT);
    assert_eq!(Direction::RIGHT.opposite(), Direction::LEFT);
    assert_eq!(Direction::TOP.opposite(), Direction::BOTTOM);
    assert_eq!(Direction::BOTTOM.opposite(), Direction::TOP);
}

#[test]
fn direction_difference() {
    assert_eq!(Direction::LEFT.difference(Direction::LEFT), 0);
    assert_eq!(Direction::RIGHT.difference(Direction::BOTTOM), 1);
    assert_eq!(Direction::BOTTOM.difference(Direction::TOP), 2);
    assert_eq!(Direction::RIGHT.difference(Direction::TOP), 3);
}

#[cfg(test)]
#[rustfmt::skip]
static TEST_TILE: [&'static str; 8] = [
    ".#......",
    "....###.",
    "......##",
    "........",
    "#.#.....",
    "..#.....",
    "........",
    ".....##.",
];

#[test]
fn tile_read() {
    let grid = read_grid(&TEST_TILE);
    let tile = Tile::new(1, &grid);
    assert_eq!(tile.id, 1);
    assert_eq!(tile.borders, [2, 4, 6, 8]);
    assert_eq!(
        tile.image,
        ndarray::arr2(&[
            [0, 0, 0, 1, 1, 1],
            [0, 0, 0, 0, 0, 1],
            [0, 0, 0, 0, 0, 0],
            [0, 1, 0, 0, 0, 0],
            [0, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0],
        ])
    );
}

#[test]
fn tile_rotate() {
    let grid = read_grid(&TEST_TILE);
    let tile = Tile::new(1, &grid).rot90(1);
    assert_eq!(tile.id, 1);
    assert_eq!(tile.borders, [4, 6, 8, 2]);
    assert_eq!(
        tile.image,
        ndarray::arr2(&[
            [1, 1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 1, 0],
            [0, 0, 0, 0, 0, 0],
        ])
    );
}

#[test]
fn tile_fliplr() {
    let grid = read_grid(&TEST_TILE);
    let tile = Tile::new(1, &grid).fliplr();
    assert_eq!(tile.id, 1);
    assert_eq!(tile.borders, [64, 16, 96, 32]);
    assert_eq!(
        tile.image,
        ndarray::arr2(&[
            [1, 1, 1, 0, 0, 0],
            [1, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 0],
            [0, 0, 0, 0, 1, 0],
            [0, 0, 0, 0, 0, 0],
        ])
    );
}

#[test]
fn tile_rotate_isomorphic() {
    let grid = read_grid(&TEST_TILE);
    let tile = Tile::new(1, &grid);
    assert_eq!(tile.rot90(0), tile);
    assert_eq!(tile.rot90(2), tile.rot90(1).rot90(1));
    assert_eq!(tile.rot90(3), tile.rot90(1).rot90(1).rot90(1));
    assert_eq!(tile.rot90(4), tile);
}
