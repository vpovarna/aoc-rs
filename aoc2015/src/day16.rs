use std::collections::HashMap;
use itertools::{enumerate, Itertools};
use aoclib::read_lines;


fn get_mfcsam_reference() -> HashMap<String, i32> {
    HashMap::from([
        ("children".to_string(), 3),
        ("cats".to_string(), 7),
        ("samoyeds".to_string(), 2),
        ("pomeranians".to_string(), 3),
        ("akitas".to_string(), 0),
        ("vizslas".to_string(), 0),
        ("goldfish".to_string(), 5),
        ("trees".to_string(), 3),
        ("cars".to_string(), 2),
        ("perfumes".to_string(), 1),
    ])
}

pub fn run() {
    let lines = read_lines("input/2015/day16.txt");
    let aunts = parse_input(lines);
    let mfcsam = get_mfcsam_reference();

    println!("Part 1: {:?}", part1(aunts, &mfcsam));
}

fn part1(aunts: Vec<HashMap<String, i32>>, mfcsam: &HashMap<String, i32>) -> usize {
    for (i, sue) in enumerate(aunts) {
        if match_sue(&sue, &mfcsam) {
            return i + 1;
        }
    }
    0
}

fn match_sue(sue: &HashMap<String, i32>, mfcsam: &HashMap<String, i32>) -> bool {
    for (key, &value) in sue {
        if let Some(&mfcsam_value) = mfcsam.get(key) {
            if mfcsam_value != value {
                return false;
            }
        }
    }
    true
}


fn parse_input(lines: Vec<String>) -> Vec<HashMap<String, i32>> {
    let mut aunts_sue: Vec<HashMap<String, i32>> = Vec::new();

    for line in lines {
        let (_, data) = line.split_once(": ").unwrap();
        let items = data.split(", ").collect_vec();
        let mut map = HashMap::new();

        for item in items {
            let (name, count) = item.split_once(": ").unwrap();
            map.insert(name.to_string(), count.parse::<i32>().unwrap());
        }

        aunts_sue.push(map)
    }

    aunts_sue
}