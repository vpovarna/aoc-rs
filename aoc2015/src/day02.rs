use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug)]
struct Present([u32; 3]);

impl Present {
    fn new(v: &[u32]) -> Self {
        Self([v[0], v[1], v[2]])
    }

    fn surface_area(&self) -> u32 {
        2 * self.0[0] * self.0[1] + 2 * self.0[0] * self.0[2] + 2 * self.0[1] * self.0[2]
    }

    fn slack(&self) -> u32 {
        self.0[0] * self.0[1]
    }

    fn ribbon(&self) -> u32 {
        self.0[0] * self.0[1] * self.0[2] + self.0[0] * 2 + self.0[1] * 2
    }
}

fn get_presents(input_file_path: impl AsRef<Path>) -> Vec<Present> {
    let input = read_to_string(input_file_path).expect("Unable to open the file");
    let mut presents_dimensions = Vec::new();

    for line in input.split('\n') {
        if !line.is_empty() {
            let iter = line.split('x');
            let mut dimensions = iter
                .map(|c| c.parse::<u32>().expect("unable to parse number"))
                .collect::<Vec<u32>>();


            dimensions.sort();
            presents_dimensions.push(Present::new(&*dimensions))
        }
    }

    presents_dimensions
}


fn part1(input_file_path: &str) -> u32 {
    get_presents(input_file_path).iter()
        .fold(0, |acc, present| acc + present.slack() + present.surface_area())
}

fn part2(input_file_path: &str) -> u32 {
    get_presents(input_file_path).iter()
        .fold(0, |acc, present| acc + present.ribbon())
}

pub fn run() {
    let input_file_path = "input/2015/day02.txt";

    println!("AoC2015, Day2, part1 solution is: {}", part1(input_file_path));
    println!("AoC2015, Day2, part2 solution is: {}", part2(input_file_path));
}

