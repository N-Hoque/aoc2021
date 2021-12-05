use self::map::Map;

pub mod point;

pub mod line;

pub mod map;

pub fn solve_sample() -> u64 {
    let map = Map::with_sample();

    map.display_intersections(false);

    let count = map.count_intersections(false);

    count as u64
}

pub fn part_1() -> u64 {
    let map = Map::new();

    map.write_intersections("res/day_5_p1_output.txt", false);

    let count = map.count_intersections(false) as u64;

    println!("Part 1: {}", count);

    count
}

pub fn solve_sample_2() -> u64 {
    let map = Map::with_sample();

    map.display_intersections(true);

    let count = map.count_intersections(true);

    count as u64
}

pub fn part_2() -> u64 {
    let map = Map::new();

    map.write_intersections("res/day_5_p2_output.txt", true);

    let count = map.count_intersections(true) as u64;

    println!("Part 2: {}", count);

    count
}

#[cfg(test)]
mod tests;
