use std::collections::HashMap;
use itertools::enumerate;
use aoclib::read_lines;

#[allow(dead_code)]
pub fn run() {
    let lines = read_lines("input/2024/day04.txt");

    println!("Part1: {}", part1(&lines));
    println!("Part2: {}", part2(&lines));
}

#[allow(dead_code)]
fn part1(lines: &Vec<String>) -> usize {
    let grid = parse_input(lines);
    let directions = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 0), (0, 1), (1, -1), (1, 0), (1, 1)];

    let mut ans = 0;
    for i in 0..lines.len() {
        for j in 0..lines.get(i).unwrap().len() {
            for direction in &directions {
                if has_xmax((i as i16, j as i16), &grid, direction) {
                    ans += 1;
                }
            }
        }
    }

    ans
}

#[allow(dead_code)]
fn part2(lines: &Vec<String>) -> usize {
    let grid = parse_input(lines);

    let mut ans = 0;
    let m = lines.len();
    let n = lines.get(0).unwrap().len();

    for i in 0..m {
        for j in 0..n {
            if has_xmax_on_diagonals((i as i16, j as i16), m as i16, n as i16, &grid) {
                ans += 1;
            }
        }
    }

    ans
}

#[allow(dead_code)]
fn has_xmax(coordinate: (i16, i16), grid: &HashMap<(i16, i16), char>, direction: &(i16, i16)) -> bool {
    for (i, c) in enumerate("XMAS".chars()) {
        let new_x = coordinate.0 + i as i16 * direction.0;
        let new_y = coordinate.1 + i as i16 * direction.1;

        if !grid.contains_key(&(new_x, new_y)) {
            return false;
        }

        if c != *grid.get(&(new_x, new_y)).unwrap() {
            return false;
        }
    }

    true
}

#[allow(dead_code)]
fn has_xmax_on_diagonals(coordinate: (i16, i16), m: i16, n: i16, grid: &HashMap<(i16, i16), char>) -> bool {
    let (i, j) = coordinate;

    if !(1 <= i && i < m - 1 && 1 <= j && j < n - 1) {
        return false;
    }
    if *grid.get(&coordinate).unwrap() != 'A' {
        return false;
    }

    let diag_1 = format!("{}{}", grid.get(&(i - 1, j - 1)).unwrap(), grid.get(&(i + 1, j + 1)).unwrap());
    let diag_2 = format!("{}{}", grid.get(&(i - 1, j + 1)).unwrap(), grid.get(&(i + 1, j - 1)).unwrap());

    (diag_1 == "MS" || diag_1 == "SM") && (diag_2 == "MS" || diag_2 == "SM")
}


#[allow(dead_code)]
fn parse_input(lines: &Vec<String>) -> HashMap<(i16, i16), char> {
    let mut grid = HashMap::new();

    for i in 0..lines.len() {
        let current_line = lines.get(i).unwrap();
        let chars: Vec<char> = current_line.chars().collect();
        for j in 0..current_line.len() {
            let c = chars.get(j).unwrap();
            grid.insert((i as i16, j as i16), *c);
        }
    }

    return grid;
}