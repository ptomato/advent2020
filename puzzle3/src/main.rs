use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut part2 = false;
    let default = String::from("1");
    let arg = args.get(1).unwrap_or(&default);
    if arg == "2" {
        part2 = true;
    }
    let landscape: Vec<String> = read_lines("input")
        .expect("Bad file")
        .map(|s| s.expect("Bad line in file"))
        .collect();

    if part2 {
        let total = count_trees_hit(&landscape, 1, 1)
            * count_trees_hit(&landscape, 3, 1)
            * count_trees_hit(&landscape, 5, 1)
            * count_trees_hit(&landscape, 7, 1)
            * count_trees_hit(&landscape, 1, 2);
        println!("{}", total);
    } else {
        let trees_hit = count_trees_hit(&landscape, 3, 1);
        println!("{}", trees_hit);
    }
}

fn count_trees_hit(landscape: &Vec<String>, right: usize, down: usize) -> usize {
    landscape
        .iter()
        .enumerate()
        .filter(|(row_index, row): &(usize, &String)| hits_tree(*row_index, row, right, down))
        .count()
}

fn hits_tree(row_index: usize, row: &String, right: usize, down: usize) -> bool {
    if row_index % down != 0 {
        return false;
    }
    let col_index = row_index / down * right % row.len();
    row.as_bytes()[col_index] == b'#'
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<path::Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
