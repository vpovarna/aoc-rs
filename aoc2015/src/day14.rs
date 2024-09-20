use itertools::Itertools;
use aoclib::read_lines;

#[derive(Clone)]
struct Reindeer {
    _name: String,
    speed: u32,
    duration: u32,
    rest: u32,
}

impl Reindeer {
    fn distance(&self, seconds: u32) -> u32 {
        let stride = self.duration + self.rest;
        let stride_count = seconds / stride;
        let remainder = seconds % stride;

        stride_count * self.speed * self.duration + self.speed * remainder.min(self.duration)
    }
}

pub fn run() {
    let lines = read_lines("input/2015/day14.txt");
    let stats = parse_input(lines);

    println!("Part 1: {}", part1(stats.clone(), 2503));
    println!("Part 2: {}", part2(stats, 2503));
}

fn parse_input(lines: Vec<String>) -> Vec<Reindeer> {
    lines.iter()
        .map(|line| {
            let parts = line.split(" ").collect_vec();
            Reindeer {
                _name: parts[0].to_string(),
                speed: parts[3].parse().unwrap(),
                duration: parts[6].parse().unwrap(),
                rest: parts[13].parse().unwrap(),
            }
        })
        .collect_vec()
}


fn part1(reindeer_stats: Vec<Reindeer>, end_time: u32) -> u32 {
    return reindeer_stats.iter().map(|d| d.distance(end_time)).max().unwrap();
}

fn part2(reindeer_stats: Vec<Reindeer>, end_time: u32) -> u32 {
    let mut far: Vec<u32> = vec![0; reindeer_stats.len()];
    for sec in 1..end_time {
        let mut furthest = 0;
        let mut max_distance = 0;

        for (i, reindeer) in reindeer_stats.iter().enumerate() {
            let dist = reindeer.distance(sec);
            if dist > max_distance {
                max_distance = dist;
                furthest = i;
            }
        }
        far[furthest] += 1
    }
    return *far.iter().max().unwrap()
}