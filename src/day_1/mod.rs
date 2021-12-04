use crate::read_file;

pub fn part_1() -> u64 {
    let data = read_file::<u64, _>("res/day_1.txt");

    let mut depth_counter = 0;
    for i in 1..data.len() {
        if data[i] > data[i - 1] {
            depth_counter += 1;
        }
    }

    println!("Part 1: {}", depth_counter);

    depth_counter
}

pub fn part_2() -> u64 {
    let data = read_file::<u64, _>("res/day_1.txt");

    let mut windows = Vec::new();
    for i in 1..data.len() - 1 {
        windows.push(data[i - 1] + data[i] + data[i + 1]);
    }

    let mut depth_counter = 0;
    for i in 1..windows.len() {
        if windows[i] > windows[i - 1] {
            depth_counter += 1;
        }
    }

    println!("Part 2: {}", depth_counter);

    depth_counter
}

#[cfg(test)]
mod tests;
