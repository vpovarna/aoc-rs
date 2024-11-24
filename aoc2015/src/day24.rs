use itertools::Itertools;
use aoclib::read_lines;

#[allow(dead_code)]
pub fn run() {
    let lines = read_lines("input/2015/day24.txt");
    let weights = lines.iter().map(|l| l.parse().unwrap()).collect_vec();

    println!("Part 1: {}", optimal_quantum_entanglement(&weights, 3));
    println!("Part 2: {}", optimal_quantum_entanglement(&weights, 4));
}

#[allow(dead_code)]
fn optimal_quantum_entanglement(weights: &Vec<usize>, compartments: usize) -> usize {
    let target_weight = weights.iter().sum::<usize>() / compartments;

    let first_compartment_loadouts = (1..=weights.len() / compartments)
        .flat_map(|compartment1_size| {
            weights.iter()
                .combinations(compartment1_size)
                .map(|c| c.into_iter().map(|w| *w).collect_vec())
                .filter(|allocated_weights|
                allocated_weights.iter().sum::<usize>() == target_weight &&
                    weights.iter()
                        .filter(|w| !allocated_weights.contains(w))
                        .sum::<usize>() == (compartments - 1) * target_weight
                ).collect_vec()
        }).collect_vec();

    let min_total_weight = first_compartment_loadouts.iter().map(|l| l.len()).min().unwrap();

    first_compartment_loadouts.iter()
        .filter(|l| l.len() == min_total_weight)
        .map(|l| l.iter().product())
        .min().unwrap()
}