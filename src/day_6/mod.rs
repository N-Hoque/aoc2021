use std::{fmt::Display, io::Read};

const SAMPLE_INPUT: [u64; 5] = [3, 4, 3, 1, 2];

#[derive(Debug, Copy, Clone)]
struct LanternFish {
    timer: u64,
}

impl LanternFish {
    fn new(timer: u64) -> Self {
        Self { timer }
    }

    fn update(&mut self) -> Option<Self> {
        if self.timer == 0 {
            self.timer = 6;
            Some(LanternFish::new(8))
        } else {
            self.timer -= 1;
            None
        }
    }
}

impl Display for LanternFish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{:>width$}]",
            self.timer,
            width = (1.0 + f64::log10(self.timer as f64).floor()) as usize
        )
    }
}

pub fn solve_sample(day_counter: usize) -> u64 {
    let mut fishes = SAMPLE_INPUT
        .into_iter()
        .map(LanternFish::new)
        .collect::<Vec<_>>();

    for day in 0..day_counter {
        let mut new_fishes = Vec::new();
        for fish in &mut fishes {
            if let Some(new_fish) = fish.update() {
                new_fishes.push(new_fish);
            }
        }
        println!("Day {}: {:?}", day, fishes);
        fishes.extend(new_fishes);
    }

    fishes.len() as u64
}

pub fn part_1() -> u64 {
    let mut data = String::new();
    let mut file = std::fs::File::open("res/day_6.txt").expect("Cannot open file");
    file.read_to_string(&mut data).expect("Cannot read file");

    let mut fishes = data
        .trim()
        .split(',')
        .map(|x| LanternFish::new(x.parse().expect("Cannot parse value")))
        .collect::<Vec<_>>();

    let mut counter = 0;

    for day in 0..80 {
        let mut new_fishes = Vec::new();
        for fish in &mut fishes {
            if let Some(new_fish) = fish.update() {
                counter += 1;
                new_fishes.push(new_fish);
            }
        }
        fishes.extend(new_fishes);
    }

    let result = fishes.len() as u64;

    println!("{}", result);

    result
}

pub fn part_2() -> u64 {
    0
}

#[cfg(test)]
mod tests;
