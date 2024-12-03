use regex::Regex;
use aoclib::read_as_string;

#[allow(dead_code)]
pub fn run() {
    let input = read_as_string("input/2024/day03.txt");
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

#[allow(dead_code)]
fn part1(input: &String) -> u32 {
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


#[allow(dead_code)]
fn part2(input: &String) -> u32 {
    // Regex patterns
    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();

    // State tracking
    let mut mul_enabled = true;
    let mut total = 0;

    // Use regex to extract meaningful tokens
    let token_re = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();

    for cap in token_re.captures_iter(&input) {
        let token = &cap[0]; // The matched token as a whole

        if do_re.is_match(token) {
            mul_enabled = true;
        } else if dont_re.is_match(token) {
            mul_enabled = false;
        } else if mul_enabled && mul_re.is_match(token) {
            total += calculate_product(token);
        }
    }

    total
}


#[allow(dead_code)]
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