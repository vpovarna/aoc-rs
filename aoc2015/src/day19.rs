use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use aoclib::read_as_string;

#[allow(dead_code)]
pub fn run() {
    println!("Part1: {}", part1())
}


#[allow(dead_code)]
fn part1() -> usize {
    let (instructions_group, sequence) = parse_input();
    assert!(instructions_group.len() > 0);
    let mut molecules: HashSet<String> = HashSet::new();

    for key in instructions_group.keys() {
        let instruction = instructions_group.get(key).unwrap();
        for (i, _) in sequence.chars().enumerate() {
            if i + key > sequence.len() {
                break;
            }
            let part = &sequence[i..i + key];
            for (from, to) in instruction {
                if part == *from {
                    let new_str = format!("{}{}{}", &sequence[..i], to, &sequence[i + key..]);
                    molecules.insert(new_str);
                }
            }
        }
    }
    molecules.len()
}


fn parse_input() -> (HashMap<usize, Vec<(String, String)>>, String) {
    let input = read_as_string("input/2015/day19.txt");
    let parts = input.split("\n\n").collect_vec();
    assert_eq!(parts.len(), 2);

    let instructions_raw = parts[0].to_string();
    let instructions = instructions_raw
        .split("\n")
        .map(|s| s.to_string())
        .collect_vec();

    let mut group_instructions: HashMap<usize, Vec<(String, String)>> = HashMap::new();

    for instruction in instructions {
        let parts = instruction.split(" => ").collect_vec();
        let key_size = parts[0].len();
        group_instructions.entry(key_size).or_insert_with(Vec::new).push((parts[0].to_string(), parts[1].to_string()));
    }

    let sequence = parts[1].to_string();

    (group_instructions, sequence)
}
