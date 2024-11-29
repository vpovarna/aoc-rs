use std::collections::HashMap;
use itertools::Itertools;
use aoclib::read_lines;

#[allow(dead_code)]
pub fn run() {
    let lines = read_lines("input/2016/day02.txt");

    println!("Part1: {:?}", part1(&lines));
    println!("Part2: {:?}", part2(&lines));
}


#[allow(dead_code)]
fn part1(lines: &Vec<String>) -> String {
    let grid = HashMap::from(
        [
            ((0, 0), "1"), ((0, 1), "2"), ((0, 2), "3"),
            ((1, 0), "4"), ((1, 1), "5"), ((1, 2), "6"),
            ((2, 0), "7"), ((2, 1), "8"), ((2, 2), "9")
        ]
    );
    let (x, y) = (1, 1);

    calculate_solution(lines, &grid, x, y)
}

#[allow(dead_code)]
fn part2(lines: &Vec<String>) -> String {
    let grid = HashMap::from(
        [
            ((0, 2), "1"),
            ((1, 1), "2"), ((1, 2), "3"), ((1, 3), "4"),
            ((2, 0), "5"), ((2, 1), "6"), ((2, 2), "7"), ((2, 3), "8"), ((2, 4), "9"),
            ((3, 1), "A"), ((3, 2), "B"), ((3, 3), "C"),
            ((4, 2), "D")
        ]);

    let (x, y) = (2, 0);

    calculate_solution(lines, &grid, x, y)
}
#[allow(dead_code)]
fn calculate_solution(lines: &Vec<String>, grid: &HashMap<(i32, i32), &str>, mut x: i32, mut y: i32) -> String {
    let mut solution: Vec<String> = Vec::new();

    for line in lines {
        let directions = line.chars().collect_vec();
        for direction in directions {
            let (new_x, new_y) = new_position(x, y, direction);
            if grid.contains_key(&(new_x, new_y)) {
                (x, y) = (new_x, new_y);
            }
        }
        let val = grid.get(&(x, y)).unwrap();
        solution.push(val.to_string());
    }
    solution.concat()
}

#[allow(dead_code)]
fn new_position(x: i32, y: i32, direction: char) -> (i32, i32) {
    let (new_x, new_y) = match direction {
        'U' => (x - 1, y),
        'D' => (x + 1, y),
        'L' => (x, y - 1),
        'R' => (x, y + 1),
        _ => panic!("Unsupported directions"),
    };
    (new_x, new_y)
}