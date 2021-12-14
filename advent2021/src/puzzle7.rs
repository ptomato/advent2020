use itertools::Itertools;
use ndarray::prelude::*;

fn fuel_cost(positions: &Array1<i32>, target: i32) -> i32 {
    (positions - target).mapv(i32::abs).sum()
}

fn minimize_fuel(positions: &Array1<i32>) -> i32 {
    let (&min, &max) = positions.iter().minmax().into_option().unwrap();
    fuel_cost(
        positions,
        (min..=max)
            .min_by_key(|&pos| fuel_cost(positions, pos))
            .unwrap(),
    )
}

pub fn main(is_part2: bool) {
    let input = include_str!("input/puzzle7");
    let positions = Array::from(
        input
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>(),
    );
    println!("{}", minimize_fuel(&positions));
}

#[test]
fn part1_example() {
    let positions = array![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    assert_eq!(fuel_cost(&positions, 1), 41);
    assert_eq!(fuel_cost(&positions, 2), 37);
    assert_eq!(fuel_cost(&positions, 3), 39);
    assert_eq!(fuel_cost(&positions, 10), 71);
    assert_eq!(minimize_fuel(&positions), 37);
}
