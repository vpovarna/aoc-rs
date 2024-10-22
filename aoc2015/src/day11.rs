use std::collections::HashSet;
use itertools::Itertools;
use aoclib::read_as_string;

#[allow(dead_code)]
pub fn run() {
    let input = read_as_string("input/2015/day11.txt");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

#[allow(dead_code)]
fn part1(input: String) -> String {
    get_next_valid_pass(input).unwrap_or(String::from("unknown"))
}

#[allow(dead_code)]
fn part2(input: String) -> String {
    let unknown_password = String::from("unknown");

    match get_next_valid_pass(input) {
        Some(password) => {
            get_next_valid_pass(incr(password.as_str())).unwrap_or(unknown_password)
        }
        None => unknown_password
    }
}

fn get_next_valid_pass(input: String) -> Option<String> {
    let mut password = input;
    while !valid(password.as_str()) {
        password = incr(password.as_str());
        if valid(password.as_str()) {
            return Some(password);
        }
    }
    None
}

fn has_no_invalid_chars(s: &str) -> bool {
    s.chars().all(|c| c != 'i' && c != 'o' && c != 'l')
}

fn incr(input: &str) -> String {
    let mut result = String::from("");
    let mut carry = true;

    for c in input.chars().rev() {
        if carry {
            if c == 'z' {
                result.push('a');
            } else {
                result.push(((c as u8) + 1) as char);
                carry = false;
            }
        } else {
            result.push(c);
        }
    }

    result.chars().rev().collect()
}

fn has_increasing_straight(s: &str) -> bool {
    s.chars().tuple_windows().any(|(a, b, c)| a as usize + 1 == b as usize && b as usize + 1 == c as usize)
}


fn has_min_two_overlapping_pairs(input_str: &str) -> bool {
    let mut iter = input_str.chars();
    let mut prev = iter.next().unwrap();
    let mut pairs = HashSet::new();

    for c in iter {
        if prev == c {
            let tmp_pair = format!("{}{}", prev, c);
            pairs.insert(tmp_pair);
        }
        prev = c;
    }

    pairs.len() >= 2
}

fn valid(input: &str) -> bool {
    has_no_invalid_chars(input) && has_increasing_straight(input) && has_min_two_overlapping_pairs(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hijklmmn_is_invalid() {
        assert!(!valid("hijklmmn"))
    }

    #[test]
    fn abbceffg_is_invalid() {
        assert!(!valid("abbceffg"))
    }

    #[test]
    fn abbcegjk_is_invalid() {
        assert!(!valid("abbcegjk"))
    }

    #[test]
    fn abcdffaa_is_valid() {
        assert!(valid("abcdffaa"))
    }

    #[test]
    fn can_increment_without_cary() {
        assert_eq!(incr("abcde"), "abcdf")
    }

    #[test]
    fn can_increment_with_cary() {
        assert_eq!(incr("abcdez"), "abcdfa")
    }
}
