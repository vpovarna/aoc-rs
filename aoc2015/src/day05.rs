use std::collections::HashSet;
use aoclib::read_lines;

use substring::Substring;

#[allow(dead_code)]
pub fn run() {
    let lines = read_lines("input/2015/day05.txt");

    println!("AoC2015, Day5, part1 solution is: {}", part1(&lines));
    println!("AoC2015, Day5, part2 solution is: {}", part2(&lines));
}

#[allow(dead_code)]
fn part1(lines: &Vec<String>) -> usize {
    let vowels = HashSet::from(['a', 'e', 'i', 'o', 'u']);
    let doubles = ('a'..='z').map(|c| format!("{}{}", c, c)).collect::<HashSet<String>>();
    let denied = HashSet::from(["ab", "cd", "pq", "xy"]);

    lines.iter()
        .filter(|&s| {
            let vowels_count = s.chars().filter(|c| vowels.contains(c)).count();
            let double_exist = doubles.iter().any(|d| s.contains(d));
            let no_denied = denied.iter().all(|&d| !s.contains(d));
            vowels_count >= 3 && double_exist && no_denied
        })
        .count()
}


fn part2(lines: &Vec<String>) -> usize {
    lines.iter()
        .filter(|&s| {
            let distance_one_repeat = (0..s.len() - 2).any(|i|
            s.chars().nth(i) == s.chars().nth(i + 2)
            );

            let pair_occurs_twice = (0..s.len() - 3).any(|start| {
                let pattern = s.substring(start, start + 2);
                s.substring(start + 2, s.len()).contains(pattern)
            });

            distance_one_repeat && pair_occurs_twice
        })
        .count()
}
