use aoclib::read_as_string;

fn part1(input_file_path: &str) -> u32 {
    let input = read_as_string(input_file_path);

    if let Some(value) = get_index(input, "00000") {
        return value;
    }
    return 0;
}

fn part2(input_file_path: &str) -> u32 {
    let input = read_as_string(input_file_path);

    if let Some(value) = get_index(input, "000000") {
        return value;
    }
    return 0;
}

fn get_index(input: String, pattern: &str) -> Option<u32> {
    let mut i = 100000;

    loop {
        let tmp_input = format!("{}{}", input, i);
        let mut md5_hash = calculate_md5_hash(tmp_input);
        if md5_hash.starts_with(pattern) {
            return Some(i);
        }
        i += 1;
    }
}

fn calculate_md5_hash(input: String) -> String {
    let md5_hash_raw = md5::compute(input);
    let md5_hash = format!("{:x}", md5_hash_raw);
    md5_hash
}

pub fn run() {
    let input_file_path = "input/2015/day04.txt";

    println!("AoC2015, Day4, part1 solution is: {}", part1(input_file_path));
    println!("AoC2015, Day4, part2 solution is: {}", part2(input_file_path));
}
