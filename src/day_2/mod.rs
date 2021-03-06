use itertools::Itertools;

use crate::read_lines;

enum Command {
    Forward(u64),
    Down(u64),
    Up(u64),
}

impl Command {
    pub fn new(command_name: &str, command_value: &str) -> Self {
        match (command_name, command_value.parse::<u64>()) {
            ("forward", Ok(x)) => Self::Forward(x),
            ("down", Ok(x)) => Self::Down(x),
            ("up", Ok(x)) => Self::Up(x),
            _ => unreachable!(),
        }
    }
}

#[derive(Default)]
struct Submarine {
    range: u64,
    depth: u64,
    aim: u64,
}

impl Submarine {
    pub fn magnitude(&self) -> u64 {
        self.range * self.depth
    }

    pub fn interpret_command_as_movement(&mut self, command: Command) {
        match command {
            Command::Forward(x) => self.range += x,
            Command::Down(x) => self.depth += x,
            Command::Up(x) => self.depth -= x,
        }
    }

    pub fn interpret_command_as_aiming(&mut self, command: Command) {
        match command {
            Command::Forward(x) => {
                self.range += x;
                self.depth += self.aim * x;
            }
            Command::Down(x) => self.aim += x,
            Command::Up(x) => self.aim -= x,
        }
    }
}

fn parse_data() -> Vec<Command> {
    let data = read_lines("res/day_2.txt");

    let commands = data
        .iter()
        .map(|x| {
            x.split_whitespace()
                .collect_tuple::<(&str, &str)>()
                .expect("Could not split line")
        })
        .map(|(name, value)| Command::new(name, value))
        .collect();

    commands
}

pub fn part_1() -> u64 {
    let mut submarine = Submarine::default();

    let commands = parse_data();

    for command in commands {
        submarine.interpret_command_as_movement(command);
    }

    println!("Part 1: {}", submarine.magnitude());

    submarine.magnitude()
}

pub fn part_2() -> u64 {
    let mut submarine = Submarine::default();

    let commands = parse_data();

    for command in commands {
        submarine.interpret_command_as_aiming(command);
    }

    println!("Part 2: {}", submarine.magnitude());

    submarine.magnitude()
}

#[cfg(test)]
mod tests;
