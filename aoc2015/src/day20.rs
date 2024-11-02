pub fn run() {
    let target = 29_000_000;
    println!("Part1: {}", part1(target));
    println!("Part2: {}", part2());
}

fn part1(target: u64) -> u64 {
    let mut limit = 1_000_000;
    let mut houses = vec![0u64; limit];

    for i in 1..limit {
        for j in (i..limit).step_by(i) {
            houses[j] += i as u64 * 10;
            if houses[j] >= target && j < limit {
                limit = j;
                break;
            }
        }
    }
    for (idx, house) in houses.iter().enumerate() {
        if *house >= target {
            return idx as u64;
        }
    }

    return 0;
}


fn part2() -> u64 {
    return 1;
}

