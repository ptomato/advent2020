#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate scan_fmt;

use bit_reverse::ParallelReverse;
use itertools::Itertools;
use ndarray::{concatenate, s, Array2, ArrayView, Axis, Ix1};
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
// use std::env;

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

bitflags! {
    struct Directions: u8 {
        const NONE = 0;
        const TOP = 0b0001;
        const RIGHT = 0b0010;
        const BOTTOM = 0b0100;
        const LEFT = 0b1000;
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
        if let Some((_, &other_side)) = other
            .borders
            .iter()
            .zip(Direction::all().iter())
            .find(|(&border, _)| border == flip_bits(border_to_connect, self.border_bits))
        {
            println!(
                "{}'s {:?} side connects to {}'s {:?} side",
                self.id, direction, other.id, other_side
            );
            Some(other_side)
        } else {
            None
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
        if tile1.connection_side(tile2).is_some() {
            let links1 = connections.entry(tile1.id).or_insert(HashSet::new());
            links1.insert(tile2.id);
            let links2 = connections.entry(tile2.id).or_insert(HashSet::new());
            links2.insert(tile1.id);
        }
    }
    connections
}

fn categorize(
    tiles: Vec<Tile>,
    connections: &HashMap<u64, HashSet<u64>>,
) -> (Vec<Tile>, Vec<Tile>, Vec<Tile>) {
    let mut corners = vec![];
    let mut edges = vec![];
    let mut centers = vec![];
    for tile in tiles {
        match connections.get(&tile.id).unwrap().len() {
            2 => corners.push(tile),
            3 => edges.push(tile),
            4 => centers.push(tile),
            _ => panic!("Impossible"),
        }
    }
    (corners, edges, centers)
}

fn orient_tile_correctly(tile: &Tile, tile_to_fit: &Tile, direction: Direction) -> Option<Tile> {
    match tile.connects_in_direction(direction, tile_to_fit) {
        None => (),
        Some(dir) => {
            let rotations = direction.opposite().difference(dir);
            println!("rotating {} by {} ccw", tile_to_fit.id, rotations);
            return Some(tile_to_fit.clone().rot90(rotations));
        }
    }
    let flipped = tile_to_fit.fliplr();
    match tile.connects_in_direction(direction, &flipped) {
        None => (),
        Some(dir) => {
            let rotations = direction.opposite().difference(dir);
            println!("flipping {} and rotating by {} ccw", flipped.id, rotations);
            return Some(flipped.rot90(rotations));
        }
    }
    None
}

fn find_and_orient_tile(
    tile: &Tile,
    possible_tiles: &[Tile],
    direction: Direction,
    connections: &HashMap<u64, HashSet<u64>>,
    used_tile_ids: &mut HashSet<u64>,
) -> Option<Tile> {
    let tile_connections = connections.get(&tile.id).unwrap();
    let candidates = possible_tiles
        .iter()
        .filter(|t| tile_connections.contains(&t.id) && !used_tile_ids.contains(&t.id));
    for candidate in candidates {
        println!(
            "candidate for connecting to {} ({:?}) is {} ({:?})",
            tile.id, tile.borders, candidate.id, candidate.borders
        );
        let next_tile = orient_tile_correctly(tile, candidate, direction);
        if let Some(t) = &next_tile {
            used_tile_ids.insert(t.id);
            return next_tile;
        }
    }
    None
}

fn arrange(
    corners: &[Tile],
    edges: &[Tile],
    centers: &[Tile],
    connections: &HashMap<u64, HashSet<u64>>,
) -> Array2<u8> {
    assert_eq!(corners.len(), 4);

    let mut used_tile_ids = HashSet::new();

    // Find top left corner - pick an arbitrary corner tile and rotate it until
    // it has connections on the right and bottom
    let mut tl_corner = corners[0].clone();
    used_tile_ids.insert(tl_corner.id);
    let mut tl_corner_connections = Directions::NONE;
    for possible_edge in edges {
        match tl_corner.connection_side(&possible_edge) {
            None => continue,
            Some(dir) if dir == Direction::TOP => tl_corner_connections |= Directions::TOP,
            Some(dir) if dir == Direction::RIGHT => tl_corner_connections |= Directions::RIGHT,
            Some(dir) if dir == Direction::BOTTOM => tl_corner_connections |= Directions::BOTTOM,
            Some(dir) if dir == Direction::LEFT => tl_corner_connections |= Directions::LEFT,
            Some(_) => panic!("Impossible"),
        }
    }
    tl_corner = tl_corner.rot90(match tl_corner_connections {
        dir if dir == Directions::RIGHT | Directions::BOTTOM => 0,
        dir if dir == Directions::BOTTOM | Directions::LEFT => 1,
        dir if dir == Directions::LEFT | Directions::TOP => 2,
        dir if dir == Directions::TOP | Directions::RIGHT => 3,
        _ => panic!("Impossible"),
    });

    // Build the top edge
    let mut t_row = vec![tl_corner];
    let mut current_tile = &t_row[t_row.len() - 1];
    loop {
        match find_and_orient_tile(
            &current_tile,
            &edges,
            Direction::RIGHT,
            connections,
            &mut used_tile_ids,
        ) {
            None => break,
            Some(tile) => {
                t_row.push(tile);
                current_tile = &t_row[t_row.len() - 1];
            }
        }
    }
    let tr_corner = find_and_orient_tile(
        &current_tile,
        &corners,
        Direction::RIGHT,
        connections,
        &mut used_tile_ids,
    )
    .unwrap();

    t_row.push(tr_corner);

    let ncols = t_row.len();
    let nrows = (corners.len() + edges.len() + centers.len()) / ncols;

    println!("whole image is {}×{}", ncols, nrows);

    // For each subsequent row except the bottom one...
    let mut rows = vec![t_row];
    for row in 1..nrows - 1 {
        // Find the left edge of the row
        let left = find_and_orient_tile(
            &rows[row - 1][0],
            &edges,
            Direction::BOTTOM,
            connections,
            &mut used_tile_ids,
        )
        .unwrap();
        let mut current_row = vec![left];
        // Arrange the middle tiles
        for col in 1..ncols - 1 {
            let next_tile = find_and_orient_tile(
                &current_row[col - 1],
                &centers,
                Direction::RIGHT,
                connections,
                &mut used_tile_ids,
            )
            .unwrap();
            current_row.push(next_tile);
        }
        // Find the right edge of the row
        let right = find_and_orient_tile(
            &current_row[ncols - 2],
            &edges,
            Direction::RIGHT,
            connections,
            &mut used_tile_ids,
        )
        .unwrap();
        current_row.push(right);

        rows.push(current_row);
    }

    // Now the bottom left corner
    let bl_corner = find_and_orient_tile(
        &rows[nrows - 2][0],
        &corners,
        Direction::BOTTOM,
        connections,
        &mut used_tile_ids,
    )
    .unwrap();
    let mut b_row = vec![bl_corner];
    // Bottom edge
    for col in 1..ncols - 1 {
        b_row.push(
            find_and_orient_tile(
                &b_row[col - 1],
                &edges,
                Direction::RIGHT,
                connections,
                &mut used_tile_ids,
            )
            .unwrap(),
        );
    }
    // Last tile
    let br_corner = find_and_orient_tile(
        &b_row[ncols - 2],
        &corners,
        Direction::RIGHT,
        connections,
        &mut used_tile_ids,
    )
    .unwrap();
    b_row.push(br_corner);
    rows.push(b_row);

    // Stack all the image data together
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

    let (corners, edges, centers) = categorize(tiles, &connections);
    assert_eq!(corners.len(), 4);
    assert_eq!(edges.len(), 4);
    assert_eq!(centers.len(), 1);

    let full_image = arrange(&corners, &edges, &centers, &connections);
    dbg!(full_image);
    todo!()
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
