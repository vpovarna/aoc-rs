use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use aoclib::read_lines;

#[allow(dead_code)]
pub fn run() {
    println!("Part1: {}", part1());
    println!("Part2: {}", part2());
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

#[allow(dead_code)]
fn part2() -> u32 {
    let (replacements, target) = parse_input();

    let mut steps = 0;
    let mut molecule = target.to_string();
    let mut reverse_replacements: Vec<(String, String)> = Vec::new();

    // Flatten the replacements into a list of (right, left) for easier random choice
    for (right, left) in replacements {
        reverse_replacements.push((right, left));
    }

    let mut rng = thread_rng();

    // Keep reducing the molecule until we get to "e"
    while molecule != "e" {
        let mut replaced = false;

        for (right, left) in &reverse_replacements {
            if let Some(pos) = molecule.find(right) {
                // Apply the replacement
                molecule.replace_range(pos..pos + right.len(), left);
                steps += 1;
                replaced = true;
                break; // Only replace one at a time
            }
        }

        if !replaced {
            // Restart if stuck in a local minimum
            molecule = target.to_string();
            steps = 0;
            reverse_replacements.shuffle(&mut rng);
        }
    }

    steps
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
