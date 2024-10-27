use std::collections::{HashMap, HashSet};
use aoclib::read_lines;

#[allow(dead_code)]
pub fn run() {
    println!("Part1: {}", part1())
}


#[allow(dead_code)]
fn part1() -> usize {
    let (replacements, molecule) = parse_input();

    let mut generated_molecules = HashSet::new();
    for (key, value) in &replacements {
        for m in molecule.match_indices(value) {
            let (left, right) = molecule.split_at(m.0);
            let right = right.to_string().split_off(value.len());
            generated_molecules.insert(format!("{left}{key}{right}"));
        }
    }
    generated_molecules.len()
}


fn parse_input() -> (HashMap<String, String>, String) {
    let input = read_lines("input/2015/day19.txt");
    let string_molecules = input.last().unwrap().to_string();
    let mut replacements: HashMap<String, String> = HashMap::new();

    for line in input {
        match line.split_once(" => ") {
            Some((left, right)) => {
                replacements.insert(right.to_string(), left.to_string());
            }
            None => continue
        }
    }
    (replacements, string_molecules)
}
