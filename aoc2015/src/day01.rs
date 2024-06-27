#[allow(dead_code)]
use aoclib::read_to_chars;

fn part1(input_file_path: &str) -> i32 {
    let chars = read_to_chars(input_file_path);
    return chars.iter().map(|x| match x {
        '(' => 1,
        ')' => -1,
        _ => 0
    }).sum::<i32>();
}

fn part2(input_file_path: &str) -> i32 {
    let chars = read_to_chars(input_file_path);
    let mut current_floor = 0;

    for (i, ch) in chars.iter().enumerate() {
        current_floor += match ch {
            '(' => 1,
            ')' => -1,
            _ => 0
        };
        if current_floor == -1 {
            return (i + 1) as i32;
        }
    }
    return -1;
}

pub fn run() {
    let input_file_path = "input/2015/day01.txt";

    println!("AoC2015, Day1, part1 solution is: {}", part1(input_file_path));
    println!("AoC2015, Day1, part2 solution is: {}", part2(input_file_path));
}