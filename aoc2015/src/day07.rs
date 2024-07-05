use std::collections::HashMap;
use aoclib::read_lines;

enum Gate {
    BUFFER(Input),
    OR(Input, Input),
    AND(Input, Input),
    NOT(Input),
    LSHIFT(Input, u16),
    RSHIFT(Input, u16),
}

enum Input {
    WIRE(String),
    SIGNAL(u16),
}

pub fn run() {
    let mut wires = parse_wires();

    let wire_a_value = wire_signal(&mut HashMap::new(), &wires, &String::from("a"));
    println!("AoC2015, Day7, part1 solution is: {}", wire_a_value);

    wires.insert(String::from("b"), Gate::BUFFER(Input::SIGNAL(wire_a_value)));
    println!("AoC2015, Day7, part2 solution is: {}", wire_signal(&mut HashMap::new(), &wires, &String::from("a")));
}


fn wire_signal(mem: &mut HashMap<String, u16>, wires: &HashMap<String, Gate>, wire: &String) -> u16 {
    match &wires[wire] {
        Gate::BUFFER(input) =>
            eval(input, wires, mem),
        Gate::NOT(input) =>
            !eval(input, wires, mem),
        Gate::AND(input1, input2) =>
            eval(input1, wires, mem) & eval(input2, wires, mem),
        Gate::OR(input1, input2) =>
            eval(input1, wires, mem) | eval(input2, wires, mem),
        Gate::LSHIFT(input, value) =>
            eval(input, wires, mem) << value,
        Gate::RSHIFT(input, value) =>
            eval(input, wires, mem) >> value
    }
}

fn eval(input: &Input, wires: &HashMap<String, Gate>, mem: &mut HashMap<String, u16>) -> u16 {
    match input {
        Input::SIGNAL(value) => *value,
        Input::WIRE(src) =>
            match mem.get(src) {
                Some(value) => *value,
                None => {
                    let result = wire_signal(mem, wires, src);
                    mem.insert(src.clone(), result);
                    result
                }
            }
    }
}

fn parse_wires() -> HashMap<String, Gate> {
    read_lines("input/2015/day07.txt")
        .iter()
        .map(|line| {
            let parts = line.split(" -> ").collect::<Vec<&str>>();
            (parts[1].to_string(), parse_gate(parts[0].split(' ').collect::<Vec<&str>>()))
        }).collect()
}

fn parse_gate(gate_vec: Vec<&str>) -> Gate {
    if gate_vec.len() == 1 {
        Gate::BUFFER(parse_input(gate_vec[0]))
    } else if gate_vec.len() == 2 {
        Gate::NOT(parse_input(gate_vec[1]))
    } else {
        match gate_vec[1] {
            "AND" => Gate::AND(parse_input(gate_vec[0]), parse_input(gate_vec[2])),
            "OR" => Gate::OR(parse_input(gate_vec[0]), parse_input(gate_vec[2])),
            "LSHIFT" => Gate::LSHIFT(parse_input(gate_vec[0]), gate_vec[2].parse().unwrap()),
            _ => Gate::RSHIFT(parse_input(gate_vec[0]), gate_vec[2].parse().unwrap())
        }
    }
}

fn parse_input(s: &str) -> Input {
    s.parse().map_or(Input::WIRE(s.to_string()), |v| Input::SIGNAL(v))
}