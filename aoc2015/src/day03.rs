use std::collections::HashSet;
use aoclib::read_to_chars;

fn part1(input_file_path: &str) -> usize {
    let directions = read_to_chars(input_file_path);
    let mut visited_houses = HashSet::new();

    let mut position = (0, 0);
    visited_houses.insert(position);

    for direction in directions {
        match direction {
            '^' => position.1 -= 1,
            'v' => position.1 += 1,
            '>' => position.0 += 1,
            '<' => position.0 -= 1,
            _ => panic!("unknown direction")
        }
        visited_houses.insert(position);
    }

    return visited_houses.len();
}

fn part2(input_file_path: &str) -> u32 {
    return 1;
}

pub fn run() {
    let input_file_path = "input/2015/day03.txt";

    println!("AoC2015, Day3, part1 solution is: {}", part1(input_file_path));
    println!("AoC2015, Day3, part2 solution is: {}", part2(input_file_path));
}

