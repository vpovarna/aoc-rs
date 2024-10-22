use aoclib::read_to_chars;
use std::collections::HashSet;

// Is it better to return a new tuple or should update the position through reference?
#[allow(dead_code)]
fn update_position(position: &(i32, i32), direction: char) -> (i32, i32) {
    let delta = match direction {
        '^' => (0, -1),
        'v' => (0, 1),
        '>' => (1, 0),
        '<' => (-1, 0),
        _ => panic!("unknown direction"),
    };
    (position.0 + delta.0, position.1 + delta.1)
}

#[allow(dead_code)]
pub fn run() {
    let input_file_path = "input/2015/day03.txt";

    println!(
        "AoC2015, Day3, part1 solution is: {}",
        part1(input_file_path)
    );
    println!(
        "AoC2015, Day3, part2 solution is: {}",
        part2(input_file_path)
    );
}

fn part1(input_file_path: &str) -> usize {
    let directions = read_to_chars(input_file_path);
    let mut visited_houses = HashSet::new();

    let mut position = (0, 0);
    visited_houses.insert(position);

    for direction in directions {
        position = update_position(&position, direction);
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
            robo_position = update_position(&robo_position, *direction);
            visited_houses.insert(robo_position);
        } else {
            santa_position = update_position(&santa_position, *direction);
            visited_houses.insert(santa_position);
        }
    }

    return visited_houses.len();
}
