use md5::compute;
use aoclib::read_as_string;

pub fn run() {
    let input = read_as_string( "input/2015/day04.txt");
    let part1_solution= part1(&input, 0);
    println!("AoC2015, Day4, part1 solution is: {}", part1_solution );
    println!("AoC2015, Day4, part2 solution is: {}", part2(&input, part1_solution));
}

fn part1(input: &String, starting_number: usize) -> usize {
    get_lowest_number(input, 5, starting_number)
}

fn part2(input: &String, starting_number: usize) -> usize {
    get_lowest_number(input, 6, starting_number)
}


fn get_lowest_number(key: &String, leading_zeros: usize, starting_number: usize) -> usize {
    let pattern = "0".repeat(leading_zeros);
    (starting_number..).find(|n| {
        let md5_hash = compute(format!("{}{}", key, n));
        let md5 = format!("{:x}", md5_hash);
        md5.starts_with(&pattern)
    }).unwrap()
}
