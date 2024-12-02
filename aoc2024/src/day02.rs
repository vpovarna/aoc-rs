use std::collections::HashSet;
use aoclib::read_lines;

#[allow(dead_code)]
pub fn run() {
    let lines = read_lines("input/2024/day02.txt");
    println!("Part1: {}", part1(&lines));
    println!("Part1: {}", part2(&lines));
}


#[allow(dead_code)]
fn part1(lines: &Vec<String>) -> u32 {
    let mut count: u32 = 0;

    for line in lines {
        let levels = parse_line(line.to_string());

        if !(is_increasing(&levels) || is_decreasing(&levels)) {
            continue;
        }

        let status = (1..levels.len())
            .map(|i| (levels[i] - levels[i - 1]).abs())
            .all(|diff| diff >= 1 && diff <= 3);

        if status {
            count += 1;
        }
    }

    return count;
}

#[allow(dead_code)]
fn part2(lines: &Vec<String>) -> usize {
    return 1;
}

#[allow(dead_code)]
fn parse_line(lines: String) -> Vec<i16> {
    lines.split(' ').map(|x| x.parse::<i16>().unwrap()).collect()
}

#[allow(dead_code)]
fn is_increasing(vec: &Vec<i16>) -> bool {
    vec.windows(2).all(|w| w[0] <= w[1])
}

#[allow(dead_code)]
fn is_decreasing(vec: &Vec<i16>) -> bool {
    vec.windows(2).all(|w| w[0] >= w[1])
}