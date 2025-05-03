use aoclib::read_lines;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn run() {
    let lines = read_lines("input/2016/day06.txt");
    println!("Part1: {}", solve(&lines, false));
    println!("Part2: {}", solve(&lines, true));
}

#[allow(dead_code)]
fn solve(lines: &Vec<String>, get_min: bool) -> String {
    let s = lines[0].len();
    let mut result = "".to_string();

    for j in 0..s {
        let mut char_occ: HashMap<char, i32> = HashMap::new();
        for i in 0..lines.len() {
            let current_line = lines.get(i).unwrap();
            let c = current_line.chars().nth(j).unwrap();
            if char_occ.contains_key(&c) {
                let value = char_occ.get(&c).unwrap() + 1;
                char_occ.insert(c, value);
            } else {
                char_occ.insert(c, 1);
            }
        }
        if get_min {
            result.push(get_least_common_char(&char_occ));
        } else {
            result.push(get_most_common_char(&char_occ));
        }
    }

    result
}

#[allow(dead_code)]
fn get_least_common_char(char_occ: &HashMap<char, i32>) -> char {
    let mut min = i32::MAX;
    let mut min_char = ' ';

    for (k, v) in char_occ {
        if *v < min {
            min = *v;
            min_char = *k;
        }
    }

    min_char
}

#[allow(dead_code)]
fn get_most_common_char(char_occ: &HashMap<char, i32>) -> char {
    let mut max = 0;
    let mut max_char = ' ';

    for (k, v) in char_occ {
        if *v > max {
            max = *v;
            max_char = *k;
        }
    }

    return max_char;
}
