use aoclib::read_lines;

#[allow(dead_code)]
pub fn run() {
    let input = read_lines("input/2022/day04.txt");
    println!("Day04, part1 is {}", part1(&input));
    println!("Day04, part2 is {}", part2(&input));
}

#[allow(dead_code)]
fn part1(input: &Vec<String>) -> usize {
    input
        .iter()
        .filter(|line| {
            let (sec1, sec2) = parse_line(&line);

            let x1 = sec1.get(0).unwrap();
            let x2 = sec1.get(1).unwrap();

            let y1 = sec2.get(0).unwrap();
            let y2 = sec2.get(1).unwrap();

            (x1 <= y1 && y1 <= y2 && y2 <= x2) || (y1 <= x1 && x1 <= x2 && x2 <= y2)
        })
        .count()
}

#[allow(dead_code)]
fn part2(input: &Vec<String>) -> usize {
    input
        .iter()
        .filter(|line| {
            let (sec1, sec2) = parse_line(&line);

            let x1 = sec1.get(0).unwrap();
            let x2 = sec1.get(1).unwrap();

            let y1 = sec2.get(0).unwrap();
            let y2 = sec2.get(1).unwrap();

            (x1 <= y1 && y2 <= x2) || (y1 <= x1 && x2 <= y2) || (x1 <= y1 && y1 <= x2) || (x1 <= y2 && y2 <= x2) || (y1 <= x1 && x1 <= y2) || (y1 <= x2 && x2 <= y2)
        })
        .count()
}

#[allow(dead_code)]
fn parse_line(line: &str) -> ([u32; 2], [u32; 2]) {
    let mut parts = line.split(',');
    let section1 = parse_section(parts.next().unwrap());
    let section2 = parse_section(parts.next().unwrap());
    (section1, section2)
}

fn parse_section(part: &str) -> [u32; 2] {
    let mut nums = part.split('-').map(|num| num.parse::<u32>().unwrap());
    [nums.next().unwrap(), nums.next().unwrap()]
}
