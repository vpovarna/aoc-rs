use aoclib::read_lines;

pub fn run() {
    let lines = read_lines("input/2015/day06.txt");

    println!("AoC2015, Day5, part1 solution is: {}", part1(&lines));
    println!("AoC2015, Day5, part2 solution is: {}", part2(&lines));
}

fn part1(raw_instructions: &Vec<String>) -> u32 {
    let instructions = parse_instructions(raw_instructions);
    for instruction in instructions {
        println!("{:?}", instruction)
    }
    return 1;
}

fn get_to(part: &str) -> Vec<usize> {
    part.split(',').map(|s| s.parse().unwrap()).collect::<Vec<usize>>()
}

fn get_from(part: &str) -> Vec<usize> {
    part.split(',').map(|s| s.parse().unwrap()).collect::<Vec<usize>>()
}

fn part2(instructions: &Vec<String>) -> u32 {
    return 1;
}

fn parse_instructions(raw_instructions: &Vec<String>) -> Vec<(String, Vec<usize>, Vec<usize>)> {
    let mut instructions = Vec::new();

    for instruction in raw_instructions {
        let parts = instruction.split(' ').collect::<Vec<&str>>();
        if parts[0] == "turn" {
            let from = get_from(parts[2]);
            let to = get_to(parts[4]);
            let s = format!("{}{}", parts[0], parts[1]);
            instructions.push((s, from, to))
            //
            // println!("{:?}", from);
            // println!("{:?}", to);
            // println!("-----");
        } else {
            let from = get_from(parts[1]);
            let to = get_from(parts[3]);
            let s = format!("{}", parts[0]);
            instructions.push((s, from, to))
        }
    }
    instructions
}

