use std::collections::HashMap;

use crate::read_to_string;

const SAMPLE_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

fn get_data(input: &str) -> Vec<u64> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse::<u64>().expect("Cannot parse value"))
        .collect()
}

fn solve(source: &str) -> u64 {
    let data = get_data(source);

    let mut costs = HashMap::new();

    for x in &data {
        if costs.contains_key(x) {
            continue;
        }
        for y in &data {
            let difference = f64::abs(*x as f64 - *y as f64) as u64;
            costs
                .entry(x)
                .and_modify(|x| *x += difference)
                .or_insert(difference);
        }
    }

    *costs.values().min().expect("No minimum found")
}

pub fn solve_sample() -> u64 {
    solve(SAMPLE_INPUT)
}

pub fn solve_sample_2() -> u64 {
    0
}

pub fn part_1() -> u64 {
    let result = solve(&read_to_string("res/day_7.txt"));

    println!("Part 1: {}", result);

    result
}

pub fn part_2() -> u64 {
    let result = solve(&read_to_string("res/day_7.txt"));

    println!("Part 2: {}", result);

    result
}

#[cfg(test)]
mod tests;
