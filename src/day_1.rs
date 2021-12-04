use crate::read_file;

pub fn part_1() {
    let data = read_file::<u64, _>("res/day_1.txt");

    let mut depth_counter = 0;
    for i in 1..data.len() {
        if data[i] > data[i - 1] {
            depth_counter += 1;
        }
    }

    println!("Part 1: {}", depth_counter);
}

#[derive(Debug, Clone, Copy)]
struct Window(u64, u64, u64);

impl Window {
    pub fn sum(self) -> u64 {
        self.0 + self.1 + self.2
    }
}

pub fn part_2() {
    let data = read_file("res/day_1.txt");

    let mut windows = Vec::new();

    for i in 1..data.len() - 1 {
        let new_window = Window(data[i - 1], data[i], data[i + 1]);
        windows.push(new_window);
    }

    let window_sizes = windows.iter().map(|w| w.sum()).collect::<Vec<_>>();

    let mut depth_counter = 0;
    for i in 1..window_sizes.len() {
        if window_sizes[i] > window_sizes[i - 1] {
            depth_counter += 1;
        }
    }

    println!("Part 2: {}", depth_counter);
}
