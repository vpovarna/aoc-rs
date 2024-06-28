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

fn part2(input_file_path: &str) -> usize {
    let directions = read_to_chars(input_file_path);
    let mut visited_houses = HashSet::new();
    visited_houses.insert((0, 0));
    let mut robo_position = (0, 0);
    let mut santa_position = (0, 0);

    for (i, direction) in directions.iter().enumerate() {
        if i % 2 == 0 {
            match direction {
                '^' => robo_position.1 -= 1,
                'v' => robo_position.1 += 1,
                '>' => robo_position.0 += 1,
                '<' => robo_position.0 -= 1,
                _ => panic!("unknown direction")
            }
            visited_houses.insert(robo_position);
        } else {
            match direction {
                '^' => santa_position.1 -= 1,
                'v' => santa_position.1 += 1,
                '>' => santa_position.0 += 1,
                '<' => santa_position.0 -= 1,
                _ => panic!("unknown direction")
            }
            visited_houses.insert(santa_position);
        }
    }

    return visited_houses.len();
}

pub fn run() {
    let input_file_path = "input/2015/day03.txt";

    println!("AoC2015, Day3, part1 solution is: {}", part1(input_file_path));
    println!("AoC2015, Day3, part2 solution is: {}", part2(input_file_path));
}

