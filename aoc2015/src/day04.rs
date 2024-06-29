use md5::compute;
use aoclib::read_as_string;

fn part1(input_file_path: &str) -> usize {
    let input = read_as_string(input_file_path);
    get_lowest_number(input, 5)
}

fn part2(input_file_path: &str) -> usize {
    let input = read_as_string(input_file_path);
    get_lowest_number(input, 6)
}


fn get_lowest_number(key: String, leading_zeros: usize) -> usize {
    let pattern = "0".repeat(leading_zeros);
    (100000..).find(|n| {
        let md5_hash = compute(format!("{}{}", key, n));
        let md5 = format!("{:x}", md5_hash);
        md5.starts_with(&pattern)
    }).unwrap()
}

pub fn run() {
    let input_file_path = "input/2015/day04.txt";

    println!("AoC2015, Day4, part1 solution is: {}", part1(input_file_path));
    println!("AoC2015, Day4, part2 solution is: {}", part2(input_file_path));
}
