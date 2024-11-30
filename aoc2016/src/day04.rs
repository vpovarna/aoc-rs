use std::collections::HashMap;
use itertools::Itertools;
use aoclib::read_lines;

pub fn run() {
    println!("Day1: {}", part1())
}

fn part1() -> i32 {
    let lines = read_lines("input/2016/day04.txt");

    let mut count: i32 = 0;
    for line in lines {
        let room = Room::new(&line);
        if room.is_real() {
            count += room.sector
        }
    }
    count
}

#[allow(dead_code)]
struct Room {
    name: String,
    sector: i32,
    checksum: String,
}

#[allow(dead_code)]
impl Room {
    fn new(room: &str) -> Self {
        let (room_str, checksum) = room.split_once('[').unwrap();
        let idx = room_str.rfind("-").unwrap();

        let (str, sector) = room_str.split_at(idx);
        let checksum = checksum.split_once(']').unwrap().0;
        let sector = -sector.parse::<i32>().unwrap();

        Self {
            name: str.to_string(),
            sector: sector,
            checksum: checksum.to_string(),
        }
    }

    fn is_real(&self) -> bool {
        let mut char_count = HashMap::new();
        let chars = self.name.chars().filter(|&c| c.is_alphabetic()).collect_vec();
        for c in chars {
            let count: &mut i32 = char_count.entry(c).or_insert(0);
            *count += 1;
        }

        // Sort the dictionary by value, then by key
        let mut list: Vec<(&char, &i32)> = char_count.iter().collect();
        list.sort_by(|a, b| {
            if a.1 == b.1 {
                a.0.cmp(b.0)
            } else {
                b.1.cmp(a.1)
            }
        });

        let str = list.iter()
            .take(5)
            .map(|c| c.0)
            .collect_vec()
            .into_iter()
            .collect::<String>();

        str == self.checksum
    }
}