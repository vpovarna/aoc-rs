use std::collections::HashMap;

use aoclib::read_lines;

const M: i32 = 100;
const N: i32 = 100;
const STEPS: i32 = 100;


#[allow(dead_code)]
pub fn run() {
    let lines = read_lines("input/2015/day18.txt");

    println!("Part1: {}", solve(lines.clone(), false));
    println!("Part2: {}", solve(lines, true));
}

#[allow(dead_code)]
fn solve(lines: Vec<String>, is_part2: bool) -> usize {
    let grid = parse_input(lines);

    (0..STEPS).fold(grid, |mut acc, _| update_grid(&mut acc, is_part2))
        .values().filter(|&l| *l == '#' as char).count()
}

fn update_grid(grid: &mut HashMap<(i32, i32), char>, part2: bool) -> HashMap<(i32, i32), char> {
    let mut new_grid: HashMap<(i32, i32), char> = HashMap::new();
    for i in 0..M {
        for j in 0..N {
            let point = (i, j);
            let neighbors_count = count_neighbors(point, &grid);
            let current_light = grid.get(&point).unwrap();
            if vec![(0, 0), (0, N - 1), (M - 1, 0), (M - 1, N - 1)].contains(&(i, j)) && part2 {
                new_grid.insert(point, '#' as char);
                continue;
            }
            if *current_light == '#' as char {
                if neighbors_count != 2 && neighbors_count != 3 {
                    new_grid.insert(point, '.' as char);
                } else {
                    new_grid.insert(point, '#' as char);
                }
            } else {
                if neighbors_count == 3 {
                    new_grid.insert(point, '#' as char);
                } else {
                    new_grid.insert(point, '.' as char);
                }
            }
        }
    }
    new_grid
}

#[allow(dead_code)]
fn parse_input(lines: Vec<String>) -> HashMap<(i32, i32), char> {
    let mut grid: HashMap<(i32, i32), char> = HashMap::new();

    for i in 0..lines.len() {
        let current_line = lines.get(i).unwrap();
        for j in 0..lines[i].len() {
            let c = current_line.chars().nth(j);
            grid.insert((i as i32, j as i32), c.unwrap());
        }
    }

    grid
}


fn get_neighbors(point: (i32, i32)) -> Vec<(i32, i32)> {
    let mut point_neighbors: Vec<(i32, i32)> = Vec::new();

    for (x, y) in vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ] {
        let new_x = point.0 + x;
        let new_y = point.1 + y;
        if new_x < 0 || new_y < 0 || new_x >= M || new_y >= N {
            continue;
        }
        point_neighbors.push((new_x, new_y));
    }
    point_neighbors
}


fn count_neighbors(point: (i32, i32), grid: &HashMap<(i32, i32), char>) -> usize {
    get_neighbors(point).into_iter()
        .map(|p| grid.get(&p).unwrap())
        .filter(|&l| *l == '#' as char)
        .count()
}

#[allow(dead_code)]
fn print_grid(grid: &HashMap<(i32, i32), char>) {
    for i in 0..M {
        for j in 0..N {
            let point = (i, j);
            print!("{}", grid.get(&point).unwrap());
        }
        println!("\n")
    }
}