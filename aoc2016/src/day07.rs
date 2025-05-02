use aoclib::read_lines;

#[allow(dead_code)]
pub fn run() {
    let lines = read_lines("input/2016/day07.txt");
    println!("Part1: {}", part1(&lines));
    println!("Part2: {}", part2(&lines));
}

#[allow(dead_code)]
fn part1(lines: &Vec<String>) -> i32 {
    let mut count = 0;

    for line in lines {
        let parsed_line = parse_line(line);

        let mut has_abba_outside = false;
        let mut has_abba_inside = false;

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

#[allow(dead_code)]
fn part2(lines: &Vec<String>) -> i32 {
    let mut count = 0;
    
    for line in lines {
        let parsed_line = parse_line(line);
        
        // Collect all ABAs from supernet segments (outside brackets)
        let mut abas: Vec<(char, char)> = Vec::new();
        
        // Collect all BABs from hypernet segments (inside brackets)
        let mut babs: Vec<(char, char)> = Vec::new();
        
        // Process segments
        for (i, segment) in parsed_line.iter().enumerate() {
            if i % 2 == 0 {
                // Supernet segment (outside brackets)
                find_abas(segment, &mut abas);
            } else {
                // Hypernet segment (inside brackets)
                find_babs(segment, &mut babs);
            }
        }
        
        // Check if any ABA has a corresponding BAB
        let mut supports_ssl = false;
        for &(a, b) in &abas {
            if babs.contains(&(b, a)) {
                supports_ssl = true;
                break;
            }
        }
        
        if supports_ssl {
            count += 1;
        }
    }
    
    count
}

#[allow(dead_code)]
fn parse_line(str: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut current_str = String::new();

    for c in str.chars() {
        match c {
            '[' => {
                // End of supernet sequence, push it to result
                result.push(current_str);
                current_str = String::new();
            }
            ']' => {
                // End of hypernet sequence, push it to result
                result.push(current_str);
                current_str = String::new();
            }
            _ => {
                current_str.push(c);
            }
        }
    }

    // Don't forget to push the last segment
    if !current_str.is_empty() {
        result.push(current_str);
    }

    return result;
}

#[allow(dead_code)]
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

// Find all ABA patterns in a string
fn find_abas(s: &str, abas: &mut Vec<(char, char)>) {
    let chars: Vec<char> = s.chars().collect();
    
    for i in 0..chars.len().saturating_sub(2) {
        let a = chars[i];
        let b = chars[i + 1];
        let c = chars[i + 2];
        
        // ABA pattern: a-b-a where a ≠ b
        if a == c && a != b {
            abas.push((a, b));
        }
    }
}

// Find all BAB patterns in a string
fn find_babs(s: &str, babs: &mut Vec<(char, char)>) {
    let chars: Vec<char> = s.chars().collect();
    
    for i in 0..chars.len().saturating_sub(2) {
        let b = chars[i];
        let a = chars[i + 1];
        let b2 = chars[i + 2];
        
        // BAB pattern: b-a-b where a ≠ b
        if b == b2 && a != b {
            babs.push((b, a));  // Store as (b, a)
        }
    }
}
