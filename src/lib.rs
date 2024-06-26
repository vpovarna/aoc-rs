use std::fs::read_to_string;


pub fn read_to_chars(file_path_name: &str) -> Vec<char> {
    let data = read_to_string(file_path_name).expect("unable to open file");

    let chars: Vec<char> = data.chars().collect();
    return chars;
}