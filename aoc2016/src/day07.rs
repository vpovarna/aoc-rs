use aoclib::read_lines;

#[allow(dead_code)]
pub fn run() {
    let lines = read_lines("input/2016/day07.txt");
    println!("Part1: {}", part1(&lines));
    // println!("Part2: {}", part2(&lines));
}

#[allow(dead_code)]
fn part1(lines: &Vec<String>) -> i32 {
    let mut count = 0;
    let mut has_abba_outside = false;
    let mut has_abba_inside = false;

    for line in lines {
        let parsed_line = parse_line(line);
        println!("{:?}", parsed_line);
        for (i, segment) in parsed_line.iter().enumerate() {
            if i % 2 == 0 {
                if has_abba(segment) {
                    has_abba_outside = true;
                }
            } else {
                if has_abba(segment) {
                    has_abba_inside = true;
                    break;
                }
            }
        }
        if has_abba_outside && !has_abba_inside {
            count += 1;
        }
    }

    count
}

fn parse_line(str: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let mut i: usize = 0;
    let mut current_str = String::new();
    let chars: Vec<char> = str.chars().collect();

    while i < chars.len() {
        let current_char: char = chars[i];
        if current_char == '[' || current_char == ']' {
            result.push(current_str);
            current_str = String::new();
        } else {
            current_str.push(current_char);
        }
        i += 1;
    }
    result.push(current_str);

    return result;
}

fn has_abba(s: &str) -> bool {
    let str_chars = s.chars().collect::<Vec<char>>();

    for i in 0..str_chars.len() - 3 {
        let a = str_chars.get(i).unwrap();
        let b = str_chars.get(i + 1).unwrap();
        let c = str_chars.get(i + 2).unwrap();
        let d: &char = str_chars.get(i + 3).unwrap();

        if a == d && b == c && a != b {
            return true;
        }
    }
    false
}
