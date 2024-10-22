use std::collections::{HashMap, HashSet};
use aoclib::read_lines;

#[allow(dead_code)]
pub fn run() {
    let lines = read_lines("input/2015/day09.txt");
    let routes = parse_input(lines);
    let adj_list = build_adjacency_list(routes);
    println!("Part 1: {}", part1(adj_list.clone()));
    println!("Part 2: {}", part2(adj_list.clone()));
}

#[allow(dead_code)]
fn part1(adj_list: HashMap<String, Vec<(String, usize)>>) -> usize {
    adj_list.keys().into_iter()
        .flat_map(|source| dfs(source, &adj_list, &mut HashSet::from([source.clone()]), true))
        .min()
        .unwrap()
}

fn part2(adj_list: HashMap<String, Vec<(String, usize)>>) -> usize {
    adj_list.keys().into_iter()
        .flat_map(|source| dfs(source, &adj_list, &mut HashSet::from([source.clone()]), false))
        .max()
        .unwrap()
}

fn dfs(
    source: &String,
    adj_list: &HashMap<String, Vec<(String, usize)>>,
    visited: &mut HashSet<String>,
    min: bool) -> Option<usize> {
    if visited.len() == adj_list.len() {
        Some(0)
    } else {
        let path_costs = adj_list[source].iter()
            .flat_map(|(dest, distance)| {
                if visited.insert(dest.clone()) {
                    let maybe_distance = dfs(&dest, adj_list, visited, min);
                    visited.remove(dest);
                    maybe_distance.map(|d| d + distance)
                } else { None }
            });

        if min { path_costs.min() } else { path_costs.max() }
    }
}

fn parse_input(input: Vec<String>) -> Vec<(String, String, usize)> {
    input.iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(" = ").collect();
            let locations: Vec<&str> = parts[0].split(" to ").collect();
            (locations[0].to_string(), locations[1].to_string(), parts[1].parse().unwrap())
        }).collect()
}

fn build_adjacency_list(routes: Vec<(String, String, usize)>) -> HashMap<String, Vec<(String, usize)>> {
    let mut adj_list: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    for (location1, location2, distance) in routes {
        let location1_destinations = adj_list.entry(location1.clone()).or_default();
        location1_destinations.push((location2.clone(), distance));
        let location2_distances = adj_list.entry(location2).or_default();
        location2_distances.push((location1, distance))
    }

    adj_list
}