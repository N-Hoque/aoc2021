use self::map::Map;

pub mod point;

pub mod line;

pub mod map;

pub fn solve_sample() -> u64 {
    let map = Map::with_sample(false);

    map.display_intersections();

    let count = map.count_intersections();

    count as u64
}

pub fn part_1() -> u64 {
    let map = Map::new(false);

    map.write_intersections("res/day_5_p1_output.txt");

    let count = map.count_intersections() as u64;

    println!("Part 1: {}", count);

    count
}

pub fn solve_sample_2() -> u64 {
    let map = Map::with_sample(true);

    map.display_intersections();

    let count = map.count_intersections();

    count as u64
}

pub fn part_2() -> u64 {
    let map = Map::new(true);

    map.write_intersections("res/day_5_p2_output.txt");

    let count = map.count_intersections() as u64;

    println!("Part 2: {}", count);

    count
}

#[cfg(test)]
mod tests;
