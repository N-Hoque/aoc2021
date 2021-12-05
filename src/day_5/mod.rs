use std::{io::Write, path::Path};

use indexmap::IndexMap;
use itertools::Itertools;

use crate::read_lines;

const SAMPLE_INPUT: [&str; 10] = [
    "0,9 -> 5,9",
    "8,0 -> 0,8",
    "9,4 -> 3,4",
    "2,2 -> 2,1",
    "7,0 -> 7,4",
    "6,4 -> 2,0",
    "0,9 -> 2,9",
    "3,4 -> 1,4",
    "0,0 -> 8,8",
    "5,5 -> 8,2",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point(u64, u64);

impl std::cmp::PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.0.cmp(&other.0) {
            std::cmp::Ordering::Less => Some(std::cmp::Ordering::Less),
            std::cmp::Ordering::Equal => match self.1.cmp(&other.1) {
                std::cmp::Ordering::Less => Some(std::cmp::Ordering::Less),
                std::cmp::Ordering::Equal => Some(std::cmp::Ordering::Equal),
                std::cmp::Ordering::Greater => Some(std::cmp::Ordering::Greater),
            },
            std::cmp::Ordering::Greater => Some(std::cmp::Ordering::Greater),
        }
    }
}

impl std::cmp::Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct LineData {
    start: Point,
    end: Point,
    points: Vec<Point>,
}

#[derive(Debug, Clone)]
pub enum Line {
    Horizontal(LineData),
    Vertical(LineData),
    Diagonal(LineData),
}

impl Line {
    fn get_all_points(&self) -> Vec<Point> {
        let mut x = vec![self.get_start()];
        x.extend(self.get_inner_data().points);
        x.push(self.get_end());
        x
    }

    fn get_inner_data(&self) -> LineData {
        match self {
            Self::Vertical(data) | Self::Horizontal(data) | Self::Diagonal(data) => data.clone(),
        }
    }

    fn get_start(&self) -> Point {
        self.get_inner_data().start
    }

    fn get_end(&self) -> Point {
        self.get_inner_data().end
    }

    fn get_points(&self) -> Vec<Point> {
        self.get_inner_data().points
    }

    fn fill_horizontal(start: &Point, end: &Point) -> Vec<Point> {
        let mut points = Vec::new();
        if start.0 < end.0 {
            for x in (start.0 + 1)..(end.0) {
                points.push(Point(x, start.1))
            }
        } else {
            for x in (end.0 + 1)..(start.0) {
                points.push(Point(x, start.1))
            }
        }
        points
    }

    fn fill_vertical(start: &Point, end: &Point) -> Vec<Point> {
        let mut points = Vec::new();
        if start.1 < end.1 {
            for y in (start.1 + 1)..(end.1) {
                points.push(Point(start.0, y))
            }
        } else {
            for y in (end.1 + 1)..(start.1) {
                points.push(Point(start.0, y))
            }
        }
        points
    }

    fn fill_diagonal(start: &Point, end: &Point) -> Vec<Point> {
        let mut points = Vec::new();

        if start.0 < end.0 && start.1 < end.1 {
            for (x, y) in ((start.0 + 1)..(end.0)).zip((start.1 + 1)..(end.1)) {
                points.push(Point(x, y))
            }
        } else if start.0 > end.0 && start.1 > end.1 {
            for (x, y) in (((end.0 + 1)..(start.0)).rev()).zip(((end.1 + 1)..(start.1)).rev()) {
                points.push(Point(x, y))
            }
        } else if start.0 > end.0 && start.1 < end.1 {
            for (x, y) in (((end.0 + 1)..(start.0)).rev()).zip((start.1 + 1)..(end.1)) {
                points.push(Point(x, y))
            }
        } else if start.0 < end.0 && start.1 > end.1 {
            for (x, y) in ((start.0 + 1)..(end.0)).zip(((end.1 + 1)..(start.1)).rev()) {
                points.push(Point(x, y))
            }
        }
        points
    }

    pub fn new(start: Point, end: Point) -> Self {
        let mut points = Vec::new();

        if start.0 == end.0 {
            points.extend(Line::fill_vertical(&start, &end));
            Self::Horizontal(LineData { start, end, points })
        } else if start.1 == end.1 {
            points.extend(Line::fill_horizontal(&start, &end));
            Self::Vertical(LineData { start, end, points })
        } else {
            points.extend(Line::fill_diagonal(&start, &end));
            Self::Diagonal(LineData { start, end, points })
        }
    }
}

struct Map {
    lines: Vec<Line>,
}

impl Map {
    fn get_all_horizontal_lines(&self) -> Vec<Line> {
        self.lines
            .iter()
            .filter(|x| matches!(x, Line::Horizontal(_)))
            .cloned()
            .collect()
    }

    fn get_all_vertical_lines(&self) -> Vec<Line> {
        self.lines
            .iter()
            .filter(|x| matches!(x, Line::Vertical(_)))
            .cloned()
            .collect()
    }

    fn get_all_diagonal_lines(&self) -> Vec<Line> {
        self.lines
            .iter()
            .filter(|x| matches!(x, Line::Diagonal(_)))
            .cloned()
            .collect()
    }

    fn get_all_points(&self) -> Vec<Point> {
        let mut points = vec![];

        for line in self.lines.iter() {
            points.extend(vec![line.get_start()]);
            points.extend(line.get_points());
            points.extend(vec![line.get_end()]);
        }

        points
    }

    fn find_minimum_bound(&self) -> Point {
        let all_points = self.get_all_points();

        all_points
            .iter()
            .min()
            .expect("Apparently no minimum exists?")
            .to_owned()
    }

    fn find_maximum_bound(&self) -> Point {
        let all_points = self.get_all_points();

        let max_x = all_points
            .iter()
            .max_by(|p1, p2| p1.0.cmp(&p2.0))
            .expect("Apparently no maximum exists?")
            .to_owned();

        let max_y = all_points
            .iter()
            .max_by(|p1, p2| p1.1.cmp(&p2.1))
            .expect("Apparently no maximum exists?")
            .to_owned();

        let max = u64::max(max_x.0, max_y.1);

        Point(max, max)
    }

    fn parse_data(data: Vec<String>) -> Vec<Line> {
        let data = data
            .iter()
            .map(|x| {
                x.split(" -> ")
                    .collect_tuple::<(&str, &str)>()
                    .expect("Cannot split by ->")
            })
            .collect::<Vec<_>>();

        let lines = data
            .iter()
            .map(|(p1, p2)| {
                let p1 = p1
                    .split(',')
                    .collect_tuple::<(&str, &str)>()
                    .map(|(x, y)| {
                        Point(
                            x.parse().expect("Cannot parse value"),
                            y.parse().expect("Cannot parse value"),
                        )
                    })
                    .expect("Cannot create point");
                let p2 = p2
                    .split(',')
                    .collect_tuple::<(&str, &str)>()
                    .map(|(x, y)| {
                        Point(
                            x.parse().expect("Cannot parse value"),
                            y.parse().expect("Cannot parse value"),
                        )
                    })
                    .expect("Cannot create point");
                Line::new(p1, p2)
            })
            .collect();

        lines
    }

    pub fn with_sample() -> Self {
        let data = SAMPLE_INPUT.iter().map(|x| x.to_string()).collect();

        let lines = Map::parse_data(data);

        Self { lines }
    }

    pub fn new() -> Self {
        let data = read_lines("res/day_5.txt");

        let lines = Map::parse_data(data);

        Self { lines }
    }

    fn get_intersections(&self, with_diagonals: bool) -> IndexMap<Point, u64> {
        let minimum_point = self.find_minimum_bound();
        let maximum_point = self.find_maximum_bound();

        let mut point_counter: IndexMap<Point, u64> = IndexMap::new();

        for x in minimum_point.0..=maximum_point.0 {
            for y in minimum_point.1..=maximum_point.1 {
                point_counter.entry(Point(y, x)).or_default();
            }
        }

        for line in self.get_all_horizontal_lines() {
            for point in line.get_all_points() {
                point_counter.entry(point).and_modify(|x| *x += 1);
            }
        }

        for line in self.get_all_vertical_lines() {
            for point in line.get_all_points() {
                point_counter.entry(point).and_modify(|x| *x += 1);
            }
        }

        if with_diagonals {
            for line in self.get_all_diagonal_lines() {
                for point in line.get_all_points() {
                    point_counter.entry(point).and_modify(|x| *x += 1);
                }
            }
        }

        point_counter
    }

    pub fn count_intersections(&self, with_diagonals: bool) -> usize {
        let straight_intersections = self.get_intersections(with_diagonals);
        straight_intersections
            .iter()
            .filter(|(_, count)| *count >= &2)
            .count()
    }

    fn draw_intersections(&self, with_diagonals: bool) -> String {
        let maximum_point = self.find_maximum_bound();

        let mut output = String::new();

        for (point, count) in self.get_intersections(with_diagonals) {
            if count == 0 {
                output += ".";
            } else {
                output += &count.to_string();
            }

            if point.0 == maximum_point.0 {
                output += "\n";
            }
        }

        output
    }

    pub fn display_intersections(&self, with_diagonals: bool) {
        println!("{}", self.draw_intersections(with_diagonals));
    }

    pub fn write_intersections<P: AsRef<Path>>(&self, out_file: P, with_diagonals: bool) {
        let intersections_string = self.draw_intersections(with_diagonals);

        let mut output_file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(out_file)
            .expect("Cannot open file");

        write!(&mut output_file, "{}", intersections_string).expect("Cannot write file");
    }
}

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
