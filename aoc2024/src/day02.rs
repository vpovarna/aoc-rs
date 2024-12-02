use aoclib::read_lines;

#[allow(dead_code)]
pub fn run() {
    let lines = read_lines("input/2024/day02.txt");
    println!("Part1: {}", part1(&lines));
    println!("Part1: {}", part2(&lines));
}

#[allow(dead_code)]
fn part1(lines: &Vec<String>) -> usize {
   lines.iter()
       .map(|line| parse_line(line.to_string()))
       .filter(|levels| is_safe(levels))
       .count()
}


#[allow(dead_code)]
fn part2(lines: &Vec<String>) -> usize {
   lines.iter()
       .map(|line| parse_line(line.to_string()))
       .filter(|levels| is_really_safe(&mut levels.clone()))
       .count()
}

#[allow(dead_code)]
fn is_really_safe(levels: &mut Vec<i16>) -> bool {
    if is_safe(levels) {
        return true;
    }

    for i in 0..levels.len() {
        let mut tmp_list = levels.clone();
        tmp_list.remove(i);
        if is_safe(&tmp_list) {
            return true;
        }
    }

    false
}

#[allow(dead_code)]
fn is_safe(levels: &Vec<i16>) -> bool {
    if !(is_increasing(&levels) || is_decreasing(&levels)) {
        return false;
    }

    (1..levels.len())
        .map(|i| (levels[i] - levels[i - 1]).abs())
        .all(|diff| diff >= 1 && diff <= 3)
}

#[allow(dead_code)]
fn parse_line(lines: String) -> Vec<i16> {
    lines.split(' ').map(|x| x.parse::<i16>().unwrap()).collect()
}

#[allow(dead_code)]
fn is_increasing(vec: &Vec<i16>) -> bool {
    vec.windows(2).all(|w| w[0] <= w[1])
}

#[allow(dead_code)]
fn is_decreasing(vec: &Vec<i16>) -> bool {
    vec.windows(2).all(|w| w[0] >= w[1])
}
