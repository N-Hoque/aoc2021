use std::collections::HashMap;

use crate::read_to_string;

const SAMPLE_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

enum Method {
    Simple,
    Summative,
}

fn get_data(input: &str) -> Vec<u64> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse::<u64>().expect("Cannot parse value"))
        .collect()
}

fn solve(source: &str, method: Method) -> u64 {
    let data = get_data(source);

    let min = *data.iter().min().expect("Cannot find minimum value");
    let max = *data.iter().max().expect("Cannot find maximum value");

    let mut minimum_cost = u64::MAX;

    for x in min..=max {
        let mut current_cost = 0;
        for y in &data {
            let cost = {
                let difference = f64::abs(x as f64 - *y as f64) as u64;
                if let Method::Simple = method {
                    difference
                } else {
                    (difference * (difference + 1)) / 2
                }
            };

            current_cost += cost;
        }

        if current_cost < minimum_cost {
            minimum_cost = current_cost;
        }
    }

    minimum_cost
}

pub fn solve_sample() -> u64 {
    solve(SAMPLE_INPUT, Method::Simple)
}

pub fn solve_sample_2() -> u64 {
    solve(SAMPLE_INPUT, Method::Summative)
}

pub fn part_1() -> u64 {
    let result = solve(&read_to_string("res/day_7.txt"), Method::Simple);

    println!("Part 1: {}", result);

    result
}

pub fn part_2() -> u64 {
    let result = solve(&read_to_string("res/day_7.txt"), Method::Summative);

    println!("Part 2: {}", result);

    result
}

#[cfg(test)]
mod tests;
