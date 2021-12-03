use std::{
    io::{BufRead, BufReader},
    path::Path,
};

pub fn read_lines<P: AsRef<Path>>(filename: P) -> Vec<String> {
    let file = std::fs::File::open(filename).expect("Unable to open file");
    let buffer = BufReader::new(file);
    buffer
        .lines()
        .map(|s| s.expect("Could not read line"))
        .collect()
}
