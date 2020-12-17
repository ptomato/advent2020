extern crate gcollections;
extern crate interval;
#[macro_use]
extern crate scan_fmt;

use gcollections::ops::set::{Contains, Union};
use interval::interval_set::{IntervalSet, ToIntervalSet};
use std::collections::HashSet;
use std::env;

// https://stackoverflow.com/a/55292215/172999
struct Multizip<T>(Vec<T>);

impl<T> Iterator for Multizip<T>
where
    T: Iterator,
{
    type Item = Vec<T::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter_mut().map(Iterator::next).collect()
    }
}

fn main() {
    let input = include_str!("input");
    let mut blocks = input.split("\n\n");

    let constraints_block = blocks.next().unwrap();
    let field_descriptions: Vec<(&str, IntervalSet<u16>)> = constraints_block
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let field_name = parts.next().unwrap();
            let interval_set = parts
                .next()
                .unwrap()
                .split(" or ")
                .map(|interval| scan_fmt!(interval, "{d}-{d}", u16, u16).unwrap())
                .collect::<Vec<(u16, u16)>>()
                .to_interval_set();
            (field_name, interval_set)
        })
        .collect();

    let mut all_valid_values = vec![].to_interval_set();
    for (_, interval) in &field_descriptions {
        all_valid_values = all_valid_values.union(interval);
    }

    let my_ticket_block = blocks.next().unwrap();
    assert!(my_ticket_block.starts_with("your ticket:\n"));

    let other_tickets_block = blocks.next().unwrap();
    assert!(other_tickets_block.starts_with("nearby tickets:\n"));
    let (valid_tickets, invalid_tickets): (Vec<Vec<u16>>, Vec<Vec<u16>>) = other_tickets_block
        .lines()
        .skip(1)
        .map(read_csv_numbers)
        .partition(|ticket| ticket.iter().all(|val| all_valid_values.contains(val)));

    if is_part2() {
        let mut possible_fields_by_position: Vec<_> = (0..valid_tickets[0].len())
            .map(|_| HashSet::new())
            .enumerate()
            .collect();
        for (position, position_values) in
            Multizip(valid_tickets.iter().map(|ticket| ticket.iter()).collect()).enumerate()
        {
            for (field_ix, (_, interval)) in field_descriptions.iter().enumerate() {
                if position_values.iter().all(|val| interval.contains(val)) {
                    possible_fields_by_position[position].1.insert(field_ix);
                }
            }
        }

        possible_fields_by_position.sort_by_key(|(_, set)| set.len());
        possible_fields_by_position.reverse();

        let mut determined_fields_by_position = vec![0; possible_fields_by_position.len()];
        while !possible_fields_by_position.is_empty() {
            let (position, possible_fields) = possible_fields_by_position.pop().unwrap();
            assert!(possible_fields.len() == 1, "unable to determine fields");
            let field_ix = possible_fields.iter().next().unwrap();
            determined_fields_by_position[position] = *field_ix;
            for (_, remaining_fields) in &mut possible_fields_by_position {
                remaining_fields.remove(field_ix);
            }
        }

        let my_ticket_values: Vec<u16> = my_ticket_block
            .lines()
            .skip(1)
            .flat_map(read_csv_numbers)
            .collect();

        let answer: u64 = determined_fields_by_position
            .iter()
            .map(|field_ix| field_descriptions[*field_ix].0)
            .zip(my_ticket_values.iter())
            .filter(|(field_name, _)| field_name.starts_with("departure"))
            .map(|(_, value)| *value as u64)
            .product();

        println!("My ticket values {:?}", answer);
    } else {
        let error_rate: u16 = invalid_tickets
            .iter()
            .flat_map(|ticket| ticket.iter().filter(|val| !all_valid_values.contains(val)))
            .sum();
        println!("Error rate: {}", error_rate);
    }
}

fn read_csv_numbers(line: &str) -> Vec<u16> {
    line.split(',').map(|s| s.parse().unwrap()).collect()
}

fn is_part2() -> bool {
    env::args().nth(1).map(|val| val == "2").unwrap_or(false)
}
