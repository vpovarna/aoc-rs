use std::collections::HashMap;
use aoclib::read_lines;

#[allow(dead_code)]
pub fn run() {
    let lines = read_lines("input/2024/day01.txt");

    println!("Part1: {}", part1(&lines));
    println!("Part1: {}", part2(&lines));
}

#[allow(dead_code)]
fn part1(lines: &Vec<String>) -> i32 {
    let (mut list1, mut list2) = parse_input(lines);

    list1.sort();
    list2.sort();

    let zip_lists = list1.iter().zip(list2.iter());
    zip_lists
        .fold(0, |acc, (&a, &b)| acc + (a - b).abs())
}

#[allow(dead_code)]
fn part2(lines: &Vec<String>) -> i32 {
    let (list1, list2) = parse_input(lines);

    let list2_occ = list2.iter()
        .fold(HashMap::new(), |mut acc, &x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        });

    let mut result = 0;

    for v in list1 {
        let solve_value = list2_occ.get(&v);
        result += v * match solve_value {
            None => 0,
            Some(&c) => c,
        }
    }
    result
}

#[allow(dead_code)]
fn parse_input(input: &Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in input.iter() {
        let mut parts = line.split("   ");
        list1.push(parts.next().unwrap().parse::<i32>().unwrap());
        list2.push(parts.next().unwrap().parse::<i32>().unwrap());
    }

    (list1, list2)
}