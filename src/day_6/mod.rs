use std::collections::HashMap;

use crate::read_to_string;

const SAMPLE_INPUT: &str = "3,4,3,1,2";

#[derive(Debug, Copy, Clone)]
struct LanternFish {
    timer: u64,
    size: u64,
}

impl LanternFish {
    fn new(timer: u64, size: u64) -> Self {
        Self { timer, size }
    }

    fn update_by(&mut self, day_skip: u64) {
        if self.timer == 0 {
            self.timer = 6;
        } else {
            self.timer -= day_skip;
        }
    }
}

struct Solver {
    fishes: Vec<LanternFish>,
}

impl Solver {
    pub fn new(source: &str) -> Self {
        Self {
            fishes: Solver::get_initial_state(&read_to_string(source)),
        }
    }

    pub fn with_sample() -> Self {
        Self {
            fishes: Solver::get_initial_state(SAMPLE_INPUT),
        }
    }

    fn get_initial_state(source: &str) -> Vec<LanternFish> {
        let fishes = {
            let source = source
                .trim()
                .split(',')
                .map(|x| x.parse::<u64>().expect("Cannot parse value"))
                .collect::<Vec<_>>();
            let mut counter = HashMap::new();
            for x in source {
                counter.entry(x).and_modify(|x| *x += 1).or_insert(1);
            }
            counter
                .into_iter()
                .map(|(timer, size)| LanternFish::new(timer, size))
                .collect::<Vec<_>>()
        };
        fishes
    }

    fn solve(&mut self, days: u64) -> u64 {
        let mut current_day = 0;
        while current_day < days {
            let min_timer = self
                .fishes
                .iter()
                .min_by(|f1, f2| f1.timer.cmp(&f2.timer))
                .unwrap()
                .timer;
            if min_timer != 0 {
                self.fishes.iter_mut().for_each(|f| f.update_by(min_timer));
                current_day += min_timer;
            } else {
                let new_fish_size = self
                    .fishes
                    .iter()
                    .filter_map(|f| if f.timer == 0 { Some(f.size) } else { None })
                    .sum();
                self.fishes.iter_mut().for_each(|f| f.update_by(1));
                self.fishes.push(LanternFish::new(8, new_fish_size));
                current_day += 1;
            }
        }

        self.fishes.iter().map(|f| f.size).sum::<u64>()
    }
}

fn solve_sample(days: u64) -> u64 {
    Solver::with_sample().solve(days)
}

pub fn part_1() -> u64 {
    let result = Solver::new("res/day_6.txt").solve(80);

    println!("{}", result);

    result
}

pub fn part_2() -> u64 {
    let result = Solver::new("res/day_6.txt").solve(256);

    println!("{}", result);

    result
}

#[cfg(test)]
mod tests;
