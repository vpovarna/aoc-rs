use std::path::Path;
use aoclib::read_lines;

pub fn run() {
    let lines = read_lines("input/2015/day06.txt");

    println!("AoC2015, Day6, part1 solution is: {}", part1(&lines));
    println!("AoC2015, Day6, part2 solution is: {}", part2(&lines));
}

fn part1(raw_instructions: &Vec<String>) -> usize {
    let instructions = parse_instructions(raw_instructions);
    let mut grid = [[false; 1000]; 1000];

    for instruction in instructions {
        let from = instruction.1;
        let to = instruction.2;

        let s = instruction.0;
        for i in (from[0]..to[0] + 1) {
            for j in from[1]..to[1] + 1 {
                grid[i][j] = match s.as_str() {
                    "turn_on" => true,
                    "turn_off" => false,
                    _ => !grid[i][j]
                }
            }
        }
    }

    let mut sum = 0;
    for line in grid.iter() {
        for &is_on in line.iter() {
            if is_on {
                sum += 1;
            }
        }
    }

    sum
}

fn part2(raw_instructions: &Vec<String>) -> usize {
    let instructions = parse_instructions(raw_instructions);
    let mut grid = [[0_usize; 1000]; 1000];

    for instruction in instructions {
        let from = instruction.1;
        let to = instruction.2;

        let s = instruction.0;
        for i in (from[0]..to[0] + 1) {
            for j in from[1]..to[1] + 1 {
                grid[i][j] = match s.as_str() {
                    "turn_on" => grid[i][j] + 1,
                    "turn_off" => grid[i][j].saturating_sub(1), //TODO: Check saturating_sub
                    _ => grid[i][j] + 2
                }
            }
        }
    }

    let mut sum = 0;
    for line in grid.iter() {
        for &light_value in line.iter() {
          sum += light_value
        }
    }

    sum
}

fn parse_instructions(raw_instructions: &Vec<String>) -> Vec<(String, Vec<usize>, Vec<usize>)> {
    let mut instructions = Vec::new();

    for instruction in raw_instructions {
        let parts = instruction.split(' ').collect::<Vec<&str>>();
        if parts[0] == "turn" {
            let from = get_from(parts[2]);
            let to = get_to(parts[4]);
            let s = format!("{}_{}", parts[0], parts[1]);
            instructions.push((s, from, to))
        } else {
            let from = get_from(parts[1]);
            let to = get_from(parts[3]);
            let s = format!("{}", parts[0]);
            instructions.push((s, from, to))
        }
    }
    instructions
}

fn get_to(part: &str) -> Vec<usize> {
    part.split(',').map(|s| s.parse().unwrap()).collect::<Vec<usize>>()
}

fn get_from(part: &str) -> Vec<usize> {
    part.split(',').map(|s| s.parse().unwrap()).collect::<Vec<usize>>()
}
