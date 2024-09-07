use std::collections::HashMap;
use itertools::Itertools;
use aoclib::read_lines;

pub fn run() {
    let lines = read_lines("input/2015/day13.txt");
    let affinities = parse_input(lines);

    println!("Part 1: {}", max_happiness(&affinities));
    // println!("Part 2: {}", part2(lines);
}

fn max_happiness(affinities: &HashMap<String, HashMap<String, i16>>) -> i16 {
    affinities.keys()
        .permutations(affinities.len())
        .map(|arrangement| find_happiness(arrangement, affinities))
        .max()
        .unwrap()
}

fn find_happiness(arrangement: Vec<&String>, affinities: &HashMap<String, HashMap<String, i16>>) -> i16 {
    arrangement.iter()
        .circular_tuple_windows()
        .map(|(&p1, &p2, &p3)| affinities[p2][p1] + affinities[p2][p3])
        .sum::<i16>()
}

fn parse_input(input: Vec<String>) -> HashMap<String, HashMap<String, i16>> {
    let mut groups: HashMap<String, HashMap<String, i16>> = HashMap::new();

    let chunks = input.into_iter()
        .map(|line| parse_line(line))
        .chunk_by(|(p1, _, _)| p1.clone());

    for (person, affinity_group) in &chunks {
        let affinity_group = affinity_group.collect_vec();
        groups.insert(person, parse_affinity(affinity_group));
    }

    return groups;
}

fn parse_affinity(affinity_group: Vec<(String, String, i16)>) -> HashMap<String, i16> {
    affinity_group.into_iter()
        .map(|(_, p, affinity)| (p, affinity))
        .collect()
}


fn parse_line(line: String) -> (String, String, i16) {
    let parts = line.split(' ').collect_vec();
    let p1 = parts[0].to_string();
    let p2 = parts[10].to_string();
    let sign = parts[2].to_string();
    let val = parts[3].parse().unwrap();
    (p1, p2, if sign == "gain" { val } else { -val })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_gain_parse_line() {
        let v = String::from("Alice would gain 54 happiness units by sitting next to Bob");
        let (pers1, pers2, happiness_level) = parse_line(v);
        assert_eq!(pers1, String::from("Alice"));
        assert_eq!(pers2, String::from("Bob"));
        assert_eq!(happiness_level, 54);
    }

    #[test]
    fn should_lose_parse_line() {
        let v = String::from("Alice would lose 62 happiness units by sitting next to Carol");
        let (pers1, pers2, happiness_level) = parse_line(v);
        assert_eq!(pers1, String::from("Alice"));
        assert_eq!(pers2, String::from("Carol"));
        assert_eq!(happiness_level, -62);
    }
}
