use std::{io::Write, path::Path};

use indexmap::IndexMap;
use itertools::Itertools;

use crate::read_lines;

use super::{line::Line, point::Point};

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

pub struct Map {
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
