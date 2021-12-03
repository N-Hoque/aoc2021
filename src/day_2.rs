use crate::read_lines;

#[derive(Debug, Clone, Copy)]
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
            _ => panic!("This cannot happen."),
        }
    }
}

#[derive(Default)]
struct Submarine {
    range: u64,
    depth: u64,
}

impl Submarine {
    pub fn magnitude(&self) -> u64 {
        self.range * self.depth
    }

    pub fn run_command(&mut self, command: Command) {
        match command {
            Command::Forward(x) => self.range += x,
            Command::Down(x) => self.depth += x,
            Command::Up(x) => self.depth -= x,
        }
    }
}

fn parse_data() -> Vec<Command> {
    let data = read_lines("res/day_2.txt");

    let mut commands = Vec::new();

    for line in data {
        let elements = line.split_whitespace().collect::<Vec<_>>();
        let command = Command::new(elements[0], elements[1]);
        commands.push(command);
    }

    commands
}

pub fn part_1() {
    let mut submarine = Submarine::default();

    let commands = parse_data();

    for command in commands {
        submarine.run_command(command);
    }

    println!("{}", submarine.magnitude());
}

pub fn part_2() {}
