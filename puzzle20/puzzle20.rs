#[macro_use]
extern crate scan_fmt;

use bit_reverse::ParallelReverse;
use itertools::Itertools;
use ndarray::{s, Array2, ArrayView, Ix1};
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
// use std::env;

#[derive(Debug, PartialEq)]
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
        let mut borders = self.borders.clone();
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

    fn flipud(&self) -> Self {
        let flipped_image = self.image.slice(s![..;-1, ..]).into_owned();
        let mut borders: Vec<_> = self
            .borders
            .iter()
            .map(|&b| flip_bits(b, self.border_bits))
            .collect();
        borders.reverse();
        borders.rotate_left(1);
        Tile {
            id: self.id,
            borders: borders.try_into().unwrap(),
            image: flipped_image,
            border_bits: self.border_bits,
        }
    }

    fn connects_to(&self, other: &Self) -> bool {
        let borders1: HashSet<_> = self.borders.iter().cloned().collect();
        let mut borders2: HashSet<_> = other.borders.iter().cloned().collect();
        borders2.extend(
            other
                .borders
                .iter()
                .map(|&b| flip_bits(b, self.border_bits)),
        );
        let intersection: HashSet<_> = borders1.intersection(&borders2).collect();
        match intersection.len() {
            0 => false,
            1 => true,
            _ => panic!(
                "Tile {} and {} can connect in more than one way",
                self.id, other.id
            ),
        }
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
    let connections = network(&tiles);

    let answer: u64 = connections
        .iter()
        .filter(|(_, links)| links.len() == 2)
        .map(|(id, _)| id)
        .product();
    println!("{}", answer);
}

fn network(tiles: &Vec<Tile>) -> HashMap<u64, HashSet<u64>> {
    let mut connections = HashMap::new();
    for (tile1, tile2) in tiles.iter().tuple_combinations() {
        if tile1.connects_to(tile2) {
            let links1 = connections.entry(tile1.id).or_insert(HashSet::new());
            links1.insert(tile2.id);
            let links2 = connections.entry(tile2.id).or_insert(HashSet::new());
            links2.insert(tile1.id);
        }
    }
    connections
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
        .filter(|s| s.len() > 0)
        .map(read_tile)
        .collect()
}

// fn is_part2() -> bool {
//     env::args().nth(1).map(|val| val == "2").unwrap_or(false)
// }

#[test]
fn example() {
    let input = include_str!("test_input");
    let tiles = read_input(input);

    assert_eq!(
        tiles.iter().map(|t| t.id).collect::<Vec<u64>>(),
        [2311, 1951, 1171, 1427, 1489, 2473, 2971, 2729, 3079]
    );

    let connections = network(&tiles);
    assert_eq!(
        tiles
            .iter()
            .map(|t| connections.get(&t.id).unwrap().len())
            .collect::<Vec<usize>>(),
        [3, 2, 2, 4, 3, 3, 2, 3, 2]
    );
}

#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
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
fn tile_flipud() {
    let grid = read_grid(&TEST_TILE);
    let tile = Tile::new(1, &grid).flipud();
    assert_eq!(tile.id, 1);
    assert_eq!(tile.borders, [96, 32, 64, 16]);
    assert_eq!(
        tile.image,
        ndarray::arr2(&[
            [0, 0, 0, 0, 0, 0],
            [0, 1, 0, 0, 0, 0],
            [0, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 1],
            [0, 0, 0, 1, 1, 1],
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
