use std::collections::HashSet;
use itertools::{enumerate, Itertools};
use aoclib::read_as_string;

#[allow(dead_code)]
pub fn run() {
    println!("Part1: {}", part1())
}


#[allow(dead_code)]
fn part1() -> usize {
    let (instructions, sequence) = parse_input();
    let mut molecules: HashSet<String> = HashSet::new();
    for instruction in instructions {
        let parts = instruction.split(" => ").collect_vec();
        let current_char = parts[0].to_string();
        let replace_sequence = parts[1].to_string();

        for (i, c) in enumerate(sequence.chars()) {
            if c == current_char.chars().next().unwrap() {
                let (part1, part2) = sequence.split_at(i);
                let part1 = &part1[..];
                let part2 = &part2[1..];
                let new_str = format!("{}{}{}", part1, replace_sequence, part2);
                molecules.insert(new_str);
            }
        }
    }
    println!("{:?}", molecules);
    molecules.len()
}


fn parse_input() -> (Vec<String>, String) {
    let input = read_as_string("input/2015/day19.txt");
    let parts = input.split("\n\n").collect_vec();
    assert_eq!(parts.len(), 2);

    let instructions_raw = parts[0].to_string();
    let instructions = instructions_raw.split("\n").map(|s| s.to_string()).collect_vec();
    let sequence = parts[1].to_string();

    (instructions, sequence)
}
