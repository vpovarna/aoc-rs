use std::collections::HashMap;
use itertools::Itertools;
use aoclib::read_as_string;

#[allow(dead_code)]
pub fn run() {
    let lines = read_as_string("input/2024/day05.txt");

    println!("Part1: {}", part1(&lines));
    println!("Part2: {}", part2(&lines));
}

#[allow(dead_code)]
fn part1(input: &String) -> usize {
    let (rules, updates) = parse_input(input);

    let mut result = 0;
    for updates in updates {
        let (is_good, mid) = follows_rules(&rules, &updates);
        if is_good {
            result += mid;
        }
    }
    result
}

#[allow(dead_code)]
fn part2(input: &String) -> usize {
    let (rules, updates) = parse_input(input);

    let mut result = 0;
    for mut update in updates {
        let (is_good, mid) = follows_rules(&rules, &update);
        if is_good {
            continue;
        }

        let ordered_seq = sort_correctly(&rules, &mut update);
        result += ordered_seq[ordered_seq.len() / 2] as usize;
    }
    result
}

#[allow(dead_code)]
fn sort_correctly(rules: &Vec<(u16, u16)>, update: &mut Vec<u16>) -> Vec<u16> {
    loop {
        let mut is_sorted = true;
        for i in 0..update.len() - 1 {
            if rules.contains(&(update[i+1], update[i])) {
                update.swap(i, i+1);
                is_sorted = false;
            }
        }
        if is_sorted {
            return update.clone();
        }
    }
}


#[allow(dead_code)]
fn follows_rules(rules: &Vec<(u16, u16)>, updates: &Vec<u16>) -> (bool, usize) {
    let mut idx_map: HashMap<u16, u16> = HashMap::new();

    for (i, num) in updates.iter().enumerate() {
        idx_map.insert(*num, i as u16);
    }

    for (a, b) in rules {
        if idx_map.contains_key(a) && idx_map.contains_key(b) && !(idx_map[a] < idx_map[b]) {
            return (false, 0);
        }
    }

    (true, updates[updates.len() / 2] as usize)
}

#[allow(dead_code)]
fn parse_input(input: &String) -> (Vec<(u16, u16)>, Vec<Vec<u16>>) {
    let (parts1, part2) = input.split_once("\n\n").unwrap();

    let mut rules: Vec<(u16, u16)> = Vec::new();

    for line in parts1.lines() {
        let v = line.split("|").collect_vec();
        rules.push((v[0].parse().unwrap(), v[1].parse().unwrap()));
    }

    let mut updates: Vec<Vec<u16>> = Vec::new();
    for line in part2.lines() {
        let tmp_vec = line.split(",").map(|x| x.parse::<u16>().unwrap()).collect_vec();
        updates.push(tmp_vec);
    }

    (rules, updates)
}