#[derive(Debug)]
enum Direction {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl Direction {
    #[cfg(test)]
    const EXAMPLE_INPUT: [Self; 6] = [
        Self::Forward(5),
        Self::Down(5),
        Self::Forward(8),
        Self::Up(3),
        Self::Down(8),
        Self::Forward(2),
    ];

    fn from_string(line: &str) -> Self {
        let (keyword, num) = scan_fmt!(line, "{} {d}", String, i32).unwrap();
        match keyword.as_str() {
            "forward" => Self::Forward(num),
            "up" => Self::Up(num),
            "down" => Self::Down(num),
            _ => panic!("Bad direction {}", line),
        }
    }
}

struct Sub {
    position: i32,
    depth: i32,
    aim: i32,
}

impl Sub {
    fn new() -> Self {
        Sub {
            position: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn go(&mut self, dir: Direction) {
        use Direction::*;
        match dir {
            Forward(dist) => self.position += dist,
            Up(dist) => self.depth -= dist,
            Down(dist) => self.depth += dist,
        }
    }

    fn move_aim(&mut self, dir: Direction) {
        use Direction::*;
        match dir {
            Forward(dist) => {
                self.position += dist;
                self.depth += dist * self.aim;
            },
            Up(dist) => self.aim -= dist,
            Down(dist) =>self.aim += dist,
        }
    }
}

pub fn main(is_part2: bool) {
    let input = include_str!("input/puzzle2");
    let mut sub = Sub::new();
    input.lines().map(Direction::from_string).for_each(|dir| {
        if is_part2 {
            sub.move_aim(dir);
        } else {
            sub.go(dir);
        }
    });
    println!("{}", sub.position * sub.depth);
}

#[test]
fn part1_example() {
    let mut sub = Sub::new();
    Direction::EXAMPLE_INPUT.into_iter().for_each(|dir| sub.go(dir));
    assert_eq!(sub.position, 15);
    assert_eq!(sub.depth, 10);
}

#[test]
fn part2_example() {
    let mut sub = Sub::new();
    Direction::EXAMPLE_INPUT.into_iter().for_each(|dir| sub.move_aim(dir));
    assert_eq!(sub.position, 15);
    assert_eq!(sub.depth, 60);
    assert_eq!(sub.aim, 10);
}
