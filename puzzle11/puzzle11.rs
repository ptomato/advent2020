use ndarray::{s, Array2};
use std::env;
use std::fs;
use std::io::{self, BufRead};

enum Tile {
    FLOOR = 0,
    SEAT = 1,
}

fn main() -> Result<(), io::Error> {
    let file = fs::File::open("input")?;
    let tiles = read_board(file);
    let mut seats = Array2::<i8>::zeros(tiles.raw_dim());

    let occupied = loop {
        let neighbours = if is_part2() {
            calc_los_neighbours(&seats, &tiles)
        } else {
            calc_neighbours(&seats)
        };
        let arrivals = (&neighbours + &seats).mapv(|count| (count == 0) as i8);
        let departures = &neighbours.mapv(if is_part2() {
            |count| (count >= 5) as i8
        } else {
            |count| (count >= 4) as i8
        }) * &seats;
        let new_seats = (&seats + &arrivals - &departures) * &tiles;
        if seats == new_seats {
            break seats.mapv(|e| e as i32).sum();
        }
        seats = new_seats;
    };

    println!("Answered: {}", occupied);
    Ok(())
}

static DIRECTIONS: &[(isize, isize)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn calc_los_neighbours(seats: &Array2<i8>, tiles: &Array2<i8>) -> Array2<i8> {
    let mut neighbours = Array2::<i8>::zeros(seats.raw_dim());
    for x in 0..seats.nrows() {
        for y in 0..seats.ncols() {
            for dir in DIRECTIONS {
                if let Some(seat) = nearest_seat(tiles, x, y, dir) {
                    neighbours[[x, y]] += seats[seat];
                }
            }
        }
    }
    neighbours
}

fn nearest_seat(
    tiles: &Array2<i8>,
    x: usize,
    y: usize,
    &(dx, dy): &(isize, isize),
) -> Option<(usize, usize)> {
    let mut nx = x;
    let mut ny = y;
    let xmax = tiles.nrows() - 1;
    let ymax = tiles.ncols() - 1;
    loop {
        if (dx == -1 && nx == 0)
            || (dx == 1 && nx >= xmax)
            || (dy == -1 && ny == 0)
            || (dy == 1 && ny >= ymax)
        {
            break None; // no seat in line of sight in this direction
        }
        nx = incdec(nx, dx);
        ny = incdec(ny, dy);
        if tiles[[nx, ny]] == Tile::SEAT as i8 {
            break Some((nx, ny));
        }
    }
}

fn incdec(val: usize, delta: isize) -> usize {
    match delta {
        1 => val + 1,
        -1 => val - 1,
        _ => val,
    }
}

fn calc_neighbours(seats: &Array2<i8>) -> Array2<i8> {
    let shape = seats.shape();
    let width = shape[0];
    let height = shape[1];
    let mut neighbours = Array2::<i8>::zeros(seats.raw_dim());
    // Add slices of the occupied seats shifted one space in each direction
    let mut slice = neighbours.slice_mut(s![1.., 1..]);
    slice += &seats.slice(s![..width - 1, ..height - 1]);
    slice = neighbours.slice_mut(s![.., 1..]);
    slice += &seats.slice(s![.., ..height - 1]);
    slice = neighbours.slice_mut(s![..width - 1, 1..]);
    slice += &seats.slice(s![1.., ..height - 1]);
    slice = neighbours.slice_mut(s![1.., ..]);
    slice += &seats.slice(s![..width - 1, ..]);
    slice = neighbours.slice_mut(s![..width - 1, ..]);
    slice += &seats.slice(s![1.., ..]);
    slice = neighbours.slice_mut(s![1.., ..height - 1]);
    slice += &seats.slice(s![..width - 1, 1..]);
    slice = neighbours.slice_mut(s![.., ..height - 1]);
    slice += &seats.slice(s![.., 1..]);
    slice = neighbours.slice_mut(s![..width - 1, ..height - 1]);
    slice += &seats.slice(s![1.., 1..]);
    neighbours
}

fn read_board(file: fs::File) -> Array2<i8> {
    let lines: Vec<String> = read_lines(file).collect();
    let height = lines.len();
    let width = lines[0].len();
    let mut cells = Array2::zeros((width, height));
    for (y, line) in lines.iter().enumerate() {
        for (x, tile) in line.bytes().enumerate() {
            cells[[x, y]] = match tile {
                b'L' => Tile::SEAT as i8,
                b'.' => Tile::FLOOR as i8,
                _ => panic!("Bad tile '{}'", tile),
            };
        }
    }
    cells
}

fn is_part2() -> bool {
    env::args().nth(1).map(|val| val == "2").unwrap_or(false)
}

fn read_lines(file: fs::File) -> impl Iterator<Item = String> {
    io::BufReader::new(file).lines().map(|res| res.unwrap())
}
