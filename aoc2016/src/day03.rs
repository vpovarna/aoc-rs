use itertools::Itertools;
use aoclib::read_lines;

#[allow(dead_code)]
pub fn run() {
    let lines = read_lines("input/2016/day03.txt");
    let triangles = parse_input(&lines);
    println!("Part1: {}", part1(&triangles));
    println!("Part2: {}", part2(&triangles));
}

type Triangle = (u32, u32, u32);

#[allow(dead_code)]
fn part1(triangles: &Vec<Triangle>) -> usize {
    triangles.iter()
        .filter(|(a, b, c)| is_valid_triangle(*a, *b, *c))
        .count()
}

#[allow(dead_code)]
fn part2(triangle: &Vec<Triangle>) -> u32 {
    let mut count: u32 = 0;
    for i in (0..triangle.len()).step_by(3) {
        count += if is_valid_triangle(triangle[i].0, triangle[i + 1].0, triangle[i + 2].0) { 1 } else { 0 };
        count += if is_valid_triangle(triangle[i].1, triangle[i + 1].1, triangle[i + 2].1) { 1 } else { 0 };
        count += if is_valid_triangle(triangle[i].2, triangle[i + 1].2, triangle[i + 2].2) { 1 } else { 0 };
    }
    count
}


#[allow(dead_code)]
fn parse_input(input: &Vec<String>) -> Vec<Triangle> {
    input.iter().map(|line| {
        let mut parts = line.split_whitespace();
        let side1 = parts.next().unwrap().parse::<u32>().unwrap();
        let side2 = parts.next().unwrap().parse::<u32>().unwrap();
        let side3 = parts.next().unwrap().parse::<u32>().unwrap();
        (side1, side2, side3)
    }).collect_vec()
}

#[allow(dead_code)]
fn is_valid_triangle(a: u32, b: u32, c: u32) -> bool {
    a + b > c && a + c > b && b + c > a
}