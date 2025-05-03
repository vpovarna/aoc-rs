use std::collections::HashMap;

use aoclib::read_lines;
use itertools::Itertools;

const WIN: i32 = 6;
const LOSS: i32 = 0;
const DRAW: i32 = 3;

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSOR: i32 = 3;

#[allow(dead_code)]
pub fn run() {
    let lines = read_lines("input/2022/day02.txt");
    let strategy = parse_input(&lines);

    println!("Day2, Part1: {}", part1(&strategy));
    println!("Day2, Part2: {}", part2(&strategy));
}

#[allow(dead_code)]
fn part1(strategy: &Vec<(char, char)>) -> i32 {
    let mut total_score = 0;
    // A,X -> Rock; B, Y -> Paper; C, Z -> Scissor

    let scores = HashMap::from([('X', 1), ('Y', 2), ('Z', 3)]);

    for (opponent_chose, player_chose) in strategy {
        match (opponent_chose, player_chose) {
            ('A', 'X') => total_score += DRAW,
            ('A', 'Y') => total_score += WIN,
            ('A', 'Z') => total_score += LOSS,

            ('B', 'X') => total_score += LOSS,
            ('B', 'Y') => total_score += DRAW,
            ('B', 'Z') => total_score += WIN,

            ('C', 'X') => total_score += WIN,
            ('C', 'Y') => total_score += LOSS,
            ('C', 'Z') => total_score += DRAW,

            _ => panic!("Unsupported hand!"),
        }
        total_score += scores.get(&player_chose).unwrap()
    }

    total_score
}

#[allow(dead_code)]
fn part2(strategy: &Vec<(char, char)>) -> i32 {
    let mut total_score = 0;
    // A,X -> Rock; B, Y -> Paper; C, Z -> Scissor

    let scores = HashMap::from([('X', 0), ('Y', 3), ('Z', 6)]);

    for (opponent_chose, player_chose) in strategy {
        match (opponent_chose, player_chose) {
            ('A', 'X') => total_score += SCISSOR,
            ('A', 'Y') => total_score += ROCK,
            ('A', 'Z') => total_score += PAPER,

            ('B', 'X') => total_score += ROCK,
            ('B', 'Y') => total_score += PAPER,
            ('B', 'Z') => total_score += SCISSOR,

            ('C', 'X') => total_score += PAPER,
            ('C', 'Y') => total_score += SCISSOR,
            ('C', 'Z') => total_score += ROCK,

            _ => panic!("Unsupported hand!"),
        }
        total_score += scores.get(&player_chose).unwrap()
    }

    total_score
}

#[allow(dead_code)]
fn parse_input(lines: &Vec<String>) -> Vec<(char, char)> {
    lines
        .iter()
        .map(|line| {
            let parts = line.split(" ").collect_vec();
            let opponent_chose = parts.get(0).unwrap().chars().next().unwrap();
            let player_chose = parts.get(1).unwrap().chars().next().unwrap();
            (opponent_chose, player_chose)
        })
        .collect_vec()
}
