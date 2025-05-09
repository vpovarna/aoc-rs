use aoclib::read_lines;
use itertools::Itertools;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[allow(dead_code)]
pub fn run() {
    let rucksack_items = read_lines("input/2022/day03.txt");
    println!("Day03, Part1: {}", part1(&rucksack_items));
    println!("Day03, Part2: {}", part2(&rucksack_items));
}

#[allow(dead_code)]
fn part1(rucksack_items: &Vec<String>) -> u32 {
    rucksack_items
        .iter()
        .map(|item| {
            let half = item.len() / 2;
            let first_compartment = &item[..half];
            let second_compartment = &item[half..];
            let common_char = find_common_char(first_compartment, second_compartment);
            match ALPHABET.find(common_char) {
                Some(value) => value as u32 + 1,
                None => 0,
            }
        })
        .sum()
}

#[allow(dead_code)]
fn part2(rucksack_items: &Vec<String>) -> u32 {
    (0..rucksack_items.len())
        .step_by(3)
        .map(|i| {
            let item_1 = rucksack_items.get(i).unwrap();
            let item_2 = rucksack_items.get(i + 1).unwrap();
            let item_3 = rucksack_items.get(i + 2).unwrap();
            let common_char = find_common_char_part2(&item_1, &item_2, &item_3);

            match ALPHABET.find(common_char) {
                Some(value) => value as u32 + 1,
                None => 0,
            }
        })
        .sum()
}

fn find_common_char_part2(item_1: &str, item_2: &str, item_3: &str) -> char {
    let item_1_chars = item_1.chars().collect_vec();
    let item_2_chars = item_2.chars().collect_vec();
    let item_3_chars = item_3.chars().collect_vec();

    for c in item_1_chars {
        if item_2_chars.contains(&c) && item_3_chars.contains(&c) {
            return c;
        }
    }
    '#'
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
