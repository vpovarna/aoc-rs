use md5;

#[allow(dead_code)]
pub fn run() {
    let input = "wtnhxymk".to_string();
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

#[allow(dead_code)]
fn part1(input_str: &str) -> String {
    let mut idx = 100000;
    let mut result = "".to_string();

    loop {
        let digest = md5::compute(format!("{}{}", input_str, idx));
        let hash = format!("{:x}", digest);
        let first_chars = &hash[0..5];
        if first_chars == "00000" {
            result.push(hash.chars().nth(5).unwrap());
            if result.len() == 8 {
                return result;
            }
        }
        idx += 1;
    }
}

#[allow(dead_code)]
fn part2(input_str: &str) -> String {
    let mut idx = 1000;
    let mut collect_vec = vec!['#'; 8];

    loop {
        let digest = md5::compute(format!("{}{}", input_str, idx));
        let hash = format!("{:x}", digest);
        let first_chars = &hash[0..5];
        if first_chars == "00000" {
            let pos = hash.chars().nth(5).unwrap().to_digit(10);
            if let Some(c) = pos {
                if c < 8 && collect_vec[c as usize] == '#' {
                    let p = hash.chars().nth(6).unwrap();
                    collect_vec[c as usize] = p;
                }
            }

            if collect_vec.contains(&'#') == false {
                let result = collect_vec.iter().collect::<String>();
                return result;
            }
        }
        idx += 1;
    }
}