use regex::Regex;
use aoclib::read_as_string;

static MULE_REGEX: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

#[allow(dead_code)]
pub fn run() {
    println!("Part1: {}", part1());
    println!("Part1: {}", part2());
}

#[allow(dead_code)]
fn part1() -> u32 {
    let input = read_as_string("input/2024/day03.txt");

    // Define the regex pattern
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    // Collect all matches
    let matches: Vec<&str> = re.find_iter(&input).map(|mat| mat.as_str()).collect();
    let mut res = 0;
    for m in matches {
        res += calculate_product(m);
    }
    res
}


fn calculate_product(mul: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    if let Some(captures) = re.captures(mul) {
        let num1: u32 = captures[1].parse().unwrap();
        let num2: u32 = captures[2].parse().unwrap();
        num1 * num2
    } else {
        1
    }
}

#[allow(dead_code)]
fn part2() -> i32 {
    0
}
