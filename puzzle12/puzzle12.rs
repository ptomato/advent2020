use std::env;
use std::fs;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Direction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl Direction {
    fn from_string(line: &str) -> Self {
        use Direction::*;
        let parameter = &line[1..];
        match line.chars().next().unwrap() {
            'N' => North(parameter.parse().unwrap()),
            'S' => South(parameter.parse().unwrap()),
            'E' => East(parameter.parse().unwrap()),
            'W' => West(parameter.parse().unwrap()),
            'L' => Left(parameter.parse().unwrap()),
            'R' => Right(parameter.parse().unwrap()),
            'F' => Forward(parameter.parse().unwrap()),
            _ => panic!("Bad instruction {}", line),
        }
    }
}

struct Ship {
    latitude: i32,  // north-south distance
    longitude: i32, // east-west distance
    facing: i32,    // east = 0, increasing clockwise, degrees / 90
    waypoint_n: i32,
    waypoint_e: i32,
}

impl Ship {
    fn new() -> Self {
        Ship {
            latitude: 0,
            longitude: 0,
            facing: 0,
            waypoint_n: 1,
            waypoint_e: 10,
        }
    }

    fn go(&mut self, dir: &Direction) {
        use Direction::*;
        match dir {
            North(dist) => self.latitude += *dist,
            South(dist) => self.latitude -= *dist,
            East(dist) => self.longitude += *dist,
            West(dist) => self.longitude -= *dist,
            Left(angle) => {
                self.facing -= *angle / 90;
                self.facing += 4;
                self.facing %= 4;
            }
            Right(angle) => {
                self.facing += *angle / 90;
                self.facing += 4;
                self.facing %= 4;
            }
            Forward(dist) => match self.facing {
                0 => self.go(&East(*dist)),
                1 => self.go(&South(*dist)),
                2 => self.go(&West(*dist)),
                3 => self.go(&North(*dist)),
                _ => panic!("Bad internal state: facing = {}", self.facing),
            },
        };
    }

    fn move_waypoint(&mut self, dir: &Direction) {
        use Direction::*;
        match dir {
            North(dist) => self.waypoint_n += *dist,
            South(dist) => self.waypoint_n -= *dist,
            East(dist) => self.waypoint_e += *dist,
            West(dist) => self.waypoint_e -= *dist,
            Left(angle) => {
                let (new_waypoint_n, new_waypoint_e) = match *angle / 90 {
                    0 => (self.waypoint_n, self.waypoint_e),
                    1 => (self.waypoint_e, -self.waypoint_n),
                    2 => (-self.waypoint_n, -self.waypoint_e),
                    3 => (-self.waypoint_e, self.waypoint_n),
                    _ => panic!("Bad angle {}", *angle),
                };
                self.waypoint_n = new_waypoint_n;
                self.waypoint_e = new_waypoint_e;
            }
            Right(angle) => {
                self.move_waypoint(&Left(360 - *angle));
            }
            Forward(times) => {
                self.latitude += self.waypoint_n * *times;
                self.longitude += self.waypoint_e * *times;
            }
        }
    }

    fn manhattan_distance(&self) -> i32 {
        self.latitude.abs() + self.longitude.abs()
    }
}

fn main() -> Result<(), io::Error> {
    let file = fs::File::open("input")?;
    let mut ship = Ship::new();
    read_lines(file)
        .map(|s| Direction::from_string(&s))
        .for_each(|dir| {
            if is_part2() {
                ship.move_waypoint(&dir)
            } else {
                ship.go(&dir)
            }
        });
    println!("{}", ship.manhattan_distance());
    Ok(())
}

fn is_part2() -> bool {
    env::args().nth(1).map(|val| val == "2").unwrap_or(false)
}

fn read_lines(file: fs::File) -> impl Iterator<Item = String> {
    io::BufReader::new(file).lines().map(|res| res.unwrap())
}
