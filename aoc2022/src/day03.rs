use aoclib::read_lines;

pub fn run() {
    let rucksack_items = read_lines("input/2022/day03.txt");
    println!("Day03, Part1: {}", part1(&rucksack_items));
}

#[allow(dead_code)]
fn part1(rucksack_items: &Vec<String>) -> u32 {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    rucksack_items
        .iter()
        .map(|item| {
            let half = item.len() / 2;
            let first_compartment = &item[..half];
            let second_compartment = &item[half..];
            let common_char = find_common_char(first_compartment, second_compartment);
            match alphabet.find(common_char) {
                Some(value) => value as u32 + 1,
                None => 0,
            }
        })
        .sum()
}

#[allow(dead_code)]
fn find_common_char(items: &str, remaining_items: &str) -> char {
    for c in items.chars() {
        if remaining_items.contains(c) {
            return c;
        }
    }
    '#'
}
