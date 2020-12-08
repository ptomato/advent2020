use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path;

#[macro_use]
extern crate scan_fmt;

fn main() -> Result<(), io::Error> {
    let mut container_rules = HashMap::new();
    let mut contents_rules = HashMap::new();
    for line in read_lines("input")?.map(|s| s.unwrap()) {
        let (adjective, color, contents) = scan_fmt!(
            &line,
            "{} {} bags contain {/[0-9a-z, ]+/}.",
            String,
            String,
            String
        )
        .unwrap_or_else(|_| panic!("{} didn't match pattern", line));
        let container = format!("{} {}", adjective, color);
        container_rules.entry(container.clone()).or_insert(vec![]);
        let contents_entry = contents_rules.entry(container.clone()).or_insert(vec![]);
        if contents != "no other bags" {
            contents
                .split(", ")
                .map(|bag| {
                    let (num, adjective, color) =
                        scan_fmt!(bag, "{d} {} {} bag", usize, String, String).unwrap_or_else(
                            |_| panic!("'{}' ({}) didn't match pattern", bag, contents),
                        );
                    (num, format!("{} {}", adjective, color))
                })
                .for_each(|(num, color)| {
                    let container_entry = container_rules.entry(color.clone()).or_insert(vec![]);
                    container_entry.push(container.clone());
                    contents_entry.push((num, color));
                });
        };
    }

    if is_part2() {
        println!("{}", total_contained_by(&contents_rules, "shiny gold"));
    } else {
        println!(
            "{}",
            all_containers_for(&container_rules, "shiny gold").len()
        );
    }

    Ok(())
}

fn total_contained_by(rules: &HashMap<String, Vec<(usize, String)>>, bag_color: &str) -> usize {
    rules[bag_color]
        .iter()
        .map(|(num, color)| num * (total_contained_by(rules, color) + 1))
        .sum()
}

fn all_containers_for(rules: &HashMap<String, Vec<String>>, bag_color: &str) -> HashSet<String> {
    let containers = &rules[bag_color];
    let mut colors: HashSet<String> = containers.iter().cloned().collect();
    for color in containers.iter() {
        let mut indirect_colors = all_containers_for(rules, color);
        colors.extend(indirect_colors.drain());
    }
    colors
}

fn is_part2() -> bool {
    env::args().nth(1).map(|val| val == "2").unwrap_or(false)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<path::Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
