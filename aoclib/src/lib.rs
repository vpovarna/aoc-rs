use std::fs::{File, read_to_string};
use std::io::{BufRead, BufReader};
use std::path::Path;


pub fn read_to_chars(file_path_name: &str) -> Vec<char> {
    read_as_string(file_path_name).chars().collect()
}

pub fn read_lines(file_path_name: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(file_path_name).expect("Unable to open file!");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}

pub fn read_as_string(file_path_name: &str) -> String {
    return read_to_string(file_path_name).expect("unable to open file");
}