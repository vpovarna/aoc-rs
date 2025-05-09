use aoclib::read_lines;
use itertools::Itertools;

pub fn run() {
    let input = read_lines("input/2022/day04.txt");
    let total = input.iter().filter(|line| {
        let (sec1, sec2) = parse_line(&line);

        let x1 = sec1.get(0).unwrap();
        let x2 = sec1.get(1).unwrap();

        let y1 = sec2.get(0).unwrap();
        let y2 = sec2.get(1).unwrap();

        (x1 <= y1 && y1 <= y2 && y2 <= x2) || (y1 <= x1 &&  x1 <= x2 && x2 <= y2)
    })
    .count();

    println!("{}", total);
}

#[allow(dead_code)]
fn parse_line(line: &String) -> (Vec<u32>, Vec<u32>) {
    let parts = line.split(',').collect_vec();
    let vec1 = get_sections(parts.get(0).unwrap());
    let vec2 = get_sections(parts.get(1).unwrap());
    (vec1, vec2)
}

fn get_sections(parts: &str) -> Vec<u32> {
    parts
        .split('-')
        .map(|a| a.parse::<u32>().unwrap())
        .collect_vec()
}
