type Population = [usize; 9];

fn init_population(fish_list: impl IntoIterator<Item = usize>) -> Population {
    let mut pop: Population = Default::default();
    fish_list.into_iter().for_each(|fish| pop[fish] += 1);
    pop
}

fn simulate_days(pop: Population, n_days: usize) -> usize {
    (0..n_days)
        .fold(pop, |p, _| {
            [p[1], p[2], p[3], p[4], p[5], p[6], p[7] + p[0], p[8], p[0]]
        })
        .iter()
        .sum::<usize>()
}

pub fn main(is_part2: bool) {
    let input = include_str!("input/puzzle6");
    let pop = init_population(input.split(',').map(|s| s.parse().unwrap()));
    println!("{}", simulate_days(pop, if is_part2 { 256 } else { 80 }));
}

#[cfg(test)]
static TEST_POPULATION: Population = [0, 1, 1, 2, 1, 0, 0, 0, 0];

#[test]
fn load_input() {
    let fish_list = [3, 4, 3, 1, 2];
    let pop = init_population(fish_list);
    assert_eq!(pop, TEST_POPULATION);
}

#[test]
fn part1_example() {
    assert_eq!(simulate_days(TEST_POPULATION, 18), 26);
    assert_eq!(simulate_days(TEST_POPULATION, 80), 5934);
}

#[test]
fn part2_example() {
    assert_eq!(simulate_days(TEST_POPULATION, 256), 26984457539);
}
