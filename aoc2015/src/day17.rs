use std::borrow::Borrow;

use aoclib::read_lines;
use itertools::Itertools;

#[allow(dead_code)]
pub fn run() {
    let lines = read_lines("./input/2015/day17.txt");
    let containers: Vec<usize> = lines.iter().map(|s| s.parse::<usize>().unwrap()).collect();

    println!("Part 1: {:?}", ways(&containers, 0, 0, 150).len());
    println!("Part 2: {}", ways_min_container(&containers, 150));
}

#[allow(dead_code)]
fn ways(containers: &Vec<usize>, idx: usize, used: usize, water: i16) -> Vec<usize> {
    if water == 0 {
        vec![used]
    } else if water < 0 || idx == containers.len() {
        vec![]
    } else {
        let with = ways(containers,  idx + 1, used + 1, water - containers[idx] as i16);
        let without = ways(containers, idx + 1, used, water);
        [with, without].concat()
    }
}

#[allow(dead_code)]
fn ways_min_container(containers: &Vec<usize>, water: i16) -> usize {
    ways(containers, 0, 0, water).into_iter()
    .into_group_map_by(|&v| v).borrow().iter()
    .map(|(&count, ways)| (count, ways.len()))
    .min_by_key(|&(v, _)| v).unwrap().1
}
