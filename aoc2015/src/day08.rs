use aoclib::read_lines;

pub fn run() {
    let lines = read_lines("input/2015/day08.txt");

    let (quote, escape, ascii) = count_special_chars(&lines);
    println!("Part 1: {}", quote + escape + ascii * 3);
    println!("Part 2: {}", lines.len() * 4 + escape * 2 + ascii);
}


fn count_special_chars(strings: &Vec<String>) -> (usize, usize, usize) {
    strings.iter().fold((0, 0, 0), |(quote, escape, ascii), s| {
        dfs(s, 0, quote, escape, ascii)
    })
}

fn dfs(s: &String, idx: usize, quote: usize, escape: usize, ascii: usize) -> (usize, usize, usize) {
    if idx == s.len() {
        (quote, escape, ascii)
    } else {
        match s.chars().nth(idx).unwrap() {
            '"' => dfs(s, idx + 1, quote + 1, escape, ascii),
            '\\' =>
                if s.chars().nth(idx + 1).unwrap() == 'x' {
                    dfs(s, idx + 4, quote, escape, ascii + 1)
                } else {
                    dfs(s, idx + 2, quote, escape + 1, ascii)
                }
            _ => dfs(s, idx + 1, quote, escape, ascii)
        }
    }
}