use crate::{read_file, read_lines};

#[derive(Default)]
struct Condition {
    gamma: u64,
    epsilon: u64,
}

impl Condition {
    pub fn consumption(&self) -> u64 {
        self.gamma * self.epsilon
    }
}

#[derive(Default, Copy, Clone)]
struct Frequency {
    zeroes: u64,
    ones: u64,
}

impl Frequency {
    pub fn max(&self) -> u64 {
        if self.zeroes > self.ones {
            0
        } else {
            1
        }
    }

    pub fn min(&self) -> u64 {
        if self.zeroes > self.ones {
            1
        } else {
            0
        }
    }
}

pub fn part_1() {
    let data = read_lines("res/day_3.txt");

    let binary_length = &data[0].len();

    let mut bit_counter = vec![Frequency::default(); *binary_length];

    for x in data {
        for (idx, c) in x
            .chars()
            .map(|x| x.to_digit(2).expect("Cannot convert char to digit"))
            .enumerate()
        {
            if c == 0 {
                bit_counter[idx].zeroes += 1;
            } else {
                bit_counter[idx].ones += 1;
            };
        }
    }

    let mut gamma = String::new();
    let mut epsilon = String::new();

    for frequency in bit_counter {
        gamma += &frequency.max().to_string();
        epsilon += &frequency.min().to_string();
    }

    println!("({}, {})", gamma, epsilon);

    let gamma = u64::from_str_radix(&gamma, 2).expect("Cannot convert to binary string");
    let epsilon = u64::from_str_radix(&epsilon, 2).expect("Cannot convert to binary string");

    let condition = Condition { gamma, epsilon };

    println!("{}", condition.consumption());
}

pub fn part_2() {
    let data = read_lines("res/day_3.txt");

    println!("{:#?}", data);
}
