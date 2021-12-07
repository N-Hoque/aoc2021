pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;

use std::{
    fmt::Debug,
    io::{BufRead, BufReader, Read},
    path::Path,
    str::FromStr,
};

pub fn read_lines<P: AsRef<Path>>(filename: P) -> Vec<String> {
    let file = std::fs::File::open(filename).expect("Unable to open file");
    let buffer = BufReader::new(file);
    buffer
        .lines()
        .map(|s| s.expect("Could not read line"))
        .collect()
}

pub fn read_file<T, P>(filename: P) -> Vec<T>
where
    T: FromStr + Debug,
    <T as FromStr>::Err: std::fmt::Debug,
    P: AsRef<Path>,
{
    let data = read_lines(filename);
    let data = data
        .iter()
        .map(|s| s.parse::<T>().expect("Could not parse value"))
        .collect::<Vec<_>>();
    data
}

pub fn read_to_string<P: AsRef<Path>>(filename: P) -> String {
    let mut data = String::new();
    let mut file = std::fs::File::open(filename).expect("Cannot open file");
    file.read_to_string(&mut data).expect("Cannot read file");
    data
}
