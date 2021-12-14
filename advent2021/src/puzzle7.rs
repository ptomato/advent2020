use itertools::Itertools;
use ndarray::prelude::*;

fn fuel_cost(positions: &Array1<i32>, target: i32) -> i32 {
    (positions - target).mapv(i32::abs).sum()
}

fn quadratic_fuel_cost(positions: &Array1<i32>, target: i32) -> i32 {
    let dist = (positions - target).mapv(i32::abs);
    (&dist * (&dist + 1) / 2).sum()
}

fn minimize_fuel(positions: &Array1<i32>, cost_function: &dyn Fn(&Array1<i32>, i32) -> i32) -> i32 {
    let (&min, &max) = positions.iter().minmax().into_option().unwrap();
    cost_function(
        positions,
        (min..=max)
            .min_by_key(|&pos| cost_function(positions, pos))
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
    println!(
        "{}",
        minimize_fuel(
            &positions,
            if is_part2 {
                &quadratic_fuel_cost
            } else {
                &fuel_cost
            }
        )
    );
}

#[test]
fn part1_example() {
    let positions = array![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    assert_eq!(fuel_cost(&positions, 1), 41);
    assert_eq!(fuel_cost(&positions, 2), 37);
    assert_eq!(fuel_cost(&positions, 3), 39);
    assert_eq!(fuel_cost(&positions, 10), 71);
    assert_eq!(minimize_fuel(&positions, &fuel_cost), 37);
}

#[test]
fn part2_example() {
    let positions = array![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    assert_eq!(quadratic_fuel_cost(&positions, 2), 206);
    assert_eq!(quadratic_fuel_cost(&positions, 5), 168);
    assert_eq!(minimize_fuel(&positions, &quadratic_fuel_cost), 168);
}
