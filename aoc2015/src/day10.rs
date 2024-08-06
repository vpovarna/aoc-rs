use itertools::Itertools;
use aoclib::read_as_string;

pub fn run() {
    let input = read_as_string("input/2015/day10.txt");
    println!("Part 1: {}", play(input.as_str(), 40));
    println!("Part 2: {}", play(input.as_str(), 50));
}

fn play(seed: &str, turns: u8) -> usize {
    if turns == 0 {
        seed.len()
    } else {
        let next_seed = seed.chars()
            .chunk_by(|&c| c)
            .into_iter()
            .map(|t| format!("{}{}", t.1.collect_vec().len(), t.0))
            .collect_vec()
            .concat();
        play(next_seed.as_str(), turns - 1)
    }
}