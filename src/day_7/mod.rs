use crate::read_to_vector;

const SAMPLE_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

enum Method {
    Simple,
    Summative,
}

struct Solver {
    data: Vec<u64>,
}

impl Solver {
    pub fn new(source: &str) -> Self {
        Self {
            data: read_to_vector::<u64, _>(source),
        }
    }

    pub fn with_sample() -> Self {
        Solver {
            data: SAMPLE_INPUT
                .split(',')
                .map(|x| x.parse::<u64>().expect("Cannot parse value"))
                .collect(),
        }
    }

    fn solve(&self, method: Method) -> u64 {
        let min = *self.data.iter().min().expect("Cannot find minimum value");
        let max = *self.data.iter().max().expect("Cannot find maximum value");

        let mut minimum_cost = u64::MAX;

        for x in min..=max {
            let current_cost = self.data.iter().fold(0, |acc, y| {
                let difference = f64::abs(x as f64 - *y as f64) as u64;
                acc + (if let Method::Simple = method {
                    difference
                } else {
                    (difference * (difference + 1)) / 2
                })
            });

            if current_cost < minimum_cost {
                minimum_cost = current_cost;
            }
        }

        minimum_cost
    }
}

pub fn solve_sample() -> u64 {
    Solver::with_sample().solve(Method::Simple)
}

pub fn solve_sample_2() -> u64 {
    Solver::with_sample().solve(Method::Summative)
}

pub fn part_1() -> u64 {
    let result = Solver::new("res/day_7.txt").solve(Method::Simple);

    println!("Part 1: {}", result);

    result
}

pub fn part_2() -> u64 {
    let result = Solver::new("res/day_7.txt").solve(Method::Summative);

    println!("Part 2: {}", result);

    result
}

#[cfg(test)]
mod tests;
