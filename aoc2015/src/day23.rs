use itertools::Itertools;
use aoclib::read_lines;
#[allow(dead_code)]
pub fn run() {
    let input = read_lines("input/2015/day23.txt");
    let instructions = parse_input(input);
    let b_register_part1 = execute(&instructions, 0, [0, 0])[1];
    let b_register_part2 = execute(&instructions, 0, [1, 0])[1];

    println!("{:?}", b_register_part1);
    println!("{:?}", b_register_part2);
}

#[allow(dead_code)]
fn execute(instructions: &Vec<Instruction>, mut index: isize, mut registers: [usize; 2]) -> [usize; 2] {
    while index >= 0 && index < instructions.iter().len() as isize {
        match instructions[index as usize] {
            Instruction::Inc(r) => registers[r] += 1,
            Instruction::Tpl(r) => registers[r] *= 3,
            Instruction::Hlf(r) => registers[r] /= 2,
            Instruction::Jmp(i) => index += i - 1,
            Instruction::Jio(r, i) => {
                if registers[r] == 1 {
                    index += i - 1
                }
            }
            Instruction::Jie(r, i) => {
                if registers[r] % 2 == 0 {
                    index += i - 1
                }
            }
        }
        index += 1;
    }

    registers
}

#[allow(dead_code)]
fn parse_input(lines: Vec<String>) -> Vec<Instruction> {
    lines.iter()
        .map(|line| {
            let parts = line.split(" ").collect_vec();
            match parts[0] {
                "inc" => Instruction::Inc(parse_register(&parts)),
                "tpl" => Instruction::Tpl(parse_register(&parts)),
                "hlf" => Instruction::Hlf(parse_register(&parts)),
                "jmp" => Instruction::Jmp(parts[1].parse().unwrap()),
                "jio" => Instruction::Jio(parse_register(&parts), parts[2].parse().unwrap()),
                _ => Instruction::Jie(parse_register(&parts), parts[2].parse().unwrap())
            }
        }).collect_vec()
}

#[allow(dead_code)]
fn parse_register(parts: &Vec<&str>) -> usize {
    if parts[1].starts_with('a') {
        0
    } else {
        1
    }
}

#[allow(dead_code)]
enum Instruction {
    Inc(usize),
    Tpl(usize),
    Hlf(usize),
    Jmp(isize),
    Jio(usize, isize),
    Jie(usize, isize),
}