use aoclib::read_as_string;
use itertools::Itertools;

#[allow(dead_code)]
pub fn run() {
    let input = read_as_string("input/2022/day01.txt");
    println!("Day1 Part1: {}", part1(&input));
    println!("Day2 Part1: {}", part2(&input));
}

#[allow(dead_code)]
fn part1(input: &String) -> i32 {
    input
        .split("\n\n")
        .map(|part| calculate_calories(part))
        .max()
        .unwrap()
}

#[allow(dead_code)]
fn part2(input: &String) -> i32 {
    input
        .split("\n\n")
        .map(|part| calculate_calories(part))
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .sum()
}

#[allow(dead_code)]
fn calculate_calories(part: &str) -> i32 {
    part.split("\n")
        .map(|calorie| calorie.parse::<i32>().unwrap())
        .sum()
}
