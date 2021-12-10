use itertools::Itertools;

fn count_increases(measurements: impl Iterator<Item=i32>) -> usize {
    measurements.tuple_windows().filter(|(a, b)| b - a > 0).count()
}

fn sum_triplets(measurements: impl Iterator<Item=i32>) -> impl Iterator<Item=i32> {
    measurements.tuple_windows().map(|(a, b, c)| (a + b + c) as i32)
}

pub fn main(is_part2: bool) {
    let input = include_str!("input/puzzle1");
    let measurements = input.lines().map(|s| s.parse().unwrap());
    let answer = if is_part2 {
        count_increases(sum_triplets(measurements))
    } else {
        count_increases(measurements)
    };
    println!("{}", answer);
}

#[cfg(test)]
static EXAMPLE_INPUT: [i32; 10] = [
    199,
    200,
    208,
    210,
    200,
    207,
    240,
    269,
    260,
    263,
];

#[test]
fn part1_example() {
    assert_eq!(count_increases(EXAMPLE_INPUT.into_iter()), 7);
}

#[test]
fn part2_example() {
    itertools::assert_equal(sum_triplets(EXAMPLE_INPUT.into_iter()), [
        607,
        618,
        618,
        617,
        647,
        716,
        769,
        792,
    ]);
    assert_eq!(count_increases(sum_triplets(EXAMPLE_INPUT.into_iter())), 5);
}
