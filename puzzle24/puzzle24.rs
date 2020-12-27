#[macro_use]
extern crate ndarray;

use multiset::HashMultiSet;
use ndarray::Array2;

type Hex = (i32, i32, i32);

#[derive(Debug, PartialEq)]
enum Direction {
    EAST,
    SOUTHEAST,
    SOUTHWEST,
    WEST,
    NORTHWEST,
    NORTHEAST,
}

impl Direction {
    fn move_rel(&self, (x, y, z): Hex) -> Hex {
        use Direction::*;
        match self {
            EAST => (x + 1, y - 1, z),
            SOUTHEAST => (x, y - 1, z + 1),
            SOUTHWEST => (x - 1, y, z + 1),
            WEST => (x - 1, y + 1, z),
            NORTHWEST => (x, y + 1, z - 1),
            NORTHEAST => (x + 1, y, z - 1),
        }
    }
}

fn parse_line(text: &str) -> Vec<Direction> {
    use Direction::*;
    let mut iter = text.bytes();
    let mut retval = Vec::with_capacity(text.len() / 2);
    while let Some(b) = iter.next() {
        retval.push(match b {
            b'e' => EAST,
            b's' => match iter.next() {
                Some(b2) if b2 == b'e' => SOUTHEAST,
                Some(b2) if b2 == b'w' => SOUTHWEST,
                Some(b2) => panic!("bad direction s{}", b2),
                None => panic!("bad direction s"),
            },
            b'w' => WEST,
            b'n' => match iter.next() {
                Some(b2) if b2 == b'w' => NORTHWEST,
                Some(b2) if b2 == b'e' => NORTHEAST,
                Some(b2) => panic!("bad direction n{}", b2),
                None => panic!("bad direction n"),
            },
            _ => panic!("bad direction {}", b),
        });
    }
    retval
}

struct Map {
    map: Array2<i8>,
    ref_q: i32,
    ref_r: i32,
}

impl Map {
    fn from_counts(counts: &HashMultiSet<Hex>) -> Self {
        let initial_extent = counts.distinct_elements().fold(0, |acc, (x, y, z)| {
            acc.max(x.abs()).max(y.abs()).max(z.abs())
        });
        let extent = initial_extent + 100; // n_iterations = 100
        let size = extent as usize;
        let map = Array2::zeros((2 * size + 1, 2 * size + 1));
        let mut this = Self {
            map,
            ref_q: extent,
            ref_r: extent,
        };
        for &(x, y, _) in counts
            .distinct_elements()
            .filter(|dest| counts.count_of(dest) % 2 == 1)
        {
            this.set(x, y);
        }
        this
    }

    fn set(&mut self, x: i32, y: i32) {
        let q = (x + self.ref_q) as usize;
        let r = (y + self.ref_r) as usize;
        self.map[[q, r]] = 1;
    }

    fn calc_neighbours(map: &Array2<i8>) -> Array2<i8> {
        let shape = map.shape();
        let width = shape[0] as isize;
        let height = shape[1] as isize;
        let mut neighbours = Array2::zeros(map.raw_dim());
        // Add slices of the occupied cells shifted one space in each hex
        // direction
        for &(xstart, ystart) in &[(1, 0), (0, 1), (-1, 1), (-1, 0), (0, -1), (1, -1)] {
            let xdest = xstart.max(0)..(width + xstart).min(width);
            let ydest = ystart.max(0)..(height + ystart).min(height);
            let xsource = (-xstart).max(0)..(width - xstart).min(width);
            let ysource = (-ystart).max(0)..(height - ystart).min(height);
            let mut slice = neighbours.slice_mut(s![xdest, ydest]);
            slice += &map.slice(s![xsource, ysource]);
        }
        neighbours
    }

    fn iterate(&mut self) {
        let neighbours = Map::calc_neighbours(&self.map);
        let removals = &neighbours.mapv(|count| (count == 0 || count > 2) as i8) * &self.map;
        let additions =
            &neighbours.mapv(|count| (count == 2) as i8) * &self.map.mapv(|cell| (cell == 0) as i8);
        self.map = &self.map + &additions - &removals;
    }

    fn count(&self) -> usize {
        self.map
            .fold(0, |acc, &cell| if cell > 0 { acc + 1 } else { acc })
    }
}

fn main() {
    let input = include_str!("input");
    let destination_counts: HashMultiSet<_> = input
        .lines()
        .map(|line| {
            parse_line(line)
                .iter()
                .fold((0, 0, 0), |hex, dir| dir.move_rel(hex))
        })
        .collect();
    let count = destination_counts
        .distinct_elements()
        .filter(|destination| destination_counts.count_of(destination) % 2 == 1)
        .count();
    println!("{}", count);

    let mut map = Map::from_counts(&destination_counts);
    for _ in 0..100 {
        map.iterate();
    }
    println!("{}", map.count());
}

#[test]
fn test_parse() {
    use Direction::*;
    let input = "esenee";
    assert_eq!(parse_line(&input), [EAST, SOUTHEAST, NORTHEAST, EAST]);
}

#[test]
fn test_move() {
    let input = parse_line("esenee");
    let end = input.iter().fold((0, 0, 0), |hex, dir| dir.move_rel(hex));
    assert_eq!(end, (3, -3, 0));
}

#[test]
fn test_example() {
    let input = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
    let destination_counts: HashMultiSet<_> = input
        .lines()
        .map(|line| {
            parse_line(line)
                .iter()
                .fold((0, 0, 0), |hex, dir| dir.move_rel(hex))
        })
        .collect();
    let mut counts = vec![0, 0, 0];
    for destination in destination_counts.distinct_elements() {
        match destination_counts.count_of(destination) {
            1 => counts[0] += 1,
            2 => counts[1] += 1,
            _ => counts[2] += 1,
        }
    }
    assert_eq!(counts, [10, 5, 0]);

    let mut map = Map::from_counts(&destination_counts);
    assert_eq!(map.count(), 10);
    let mut results: Vec<_> = (0..10)
        .map(|_| {
            map.iterate();
            map.count()
        })
        .collect();
    assert_eq!(results, [15, 12, 25, 14, 23, 28, 41, 37, 49, 37]);
    results = (1..10)
        .map(|_| {
            for _ in 0..10 {
                map.iterate();
            }
            map.count()
        })
        .collect();
    assert_eq!(results, [132, 259, 406, 566, 788, 1106, 1373, 1844, 2208]);
}
