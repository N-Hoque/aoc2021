use std::{
    io::{BufRead, Write},
    path::Path,
};

use indexmap::IndexMap;
use itertools::Itertools;

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
    point_counter: IndexMap<Point, u64>,
}

impl Map {
    pub fn new(with_diagonals: bool) -> Self {
        let mut new_map = BaseMap::init();

        new_map.find_intersections(with_diagonals);

        Self {
            point_counter: new_map.point_counter,
        }
    }

    pub fn with_sample(with_diagonals: bool) -> Self {
        let mut new_map = BaseMap::init();

        new_map.find_intersections_with_sample(with_diagonals);

        Self {
            point_counter: new_map.point_counter,
        }
    }

    pub fn count_intersections(&self) -> usize {
        self.point_counter
            .iter()
            .filter(|(_, count)| *count >= &2)
            .count()
    }

    pub fn display_intersections(&self) {
        println!("{}", self.draw_intersections());
    }

    pub fn write_intersections<P: AsRef<Path>>(&self, out_file: P) {
        let intersections_string = self.draw_intersections();

        let mut output_file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(out_file)
            .expect("Cannot open file");

        write!(&mut output_file, "{}", intersections_string).expect("Cannot write file");
    }

    fn get_min_bound(&self) -> Point {
        Point(0, 0)
    }

    fn get_max_bound(&self) -> Point {
        let max_x = self
            .point_counter
            .keys()
            .max_by(|p1, p2| p1.0.cmp(&p2.0))
            .expect("No maximum found")
            .to_owned();

        let max_y = self
            .point_counter
            .keys()
            .max_by(|p1, p2| p1.1.cmp(&p2.1))
            .expect("No maximum found")
            .to_owned();

        let max = u64::max(max_x.0, max_y.1);

        Point(max, max)
    }

    fn draw_intersections(&self) -> String {
        let minimum_point = self.get_min_bound();
        let maximum_point = self.get_max_bound();

        let mut output = String::new();

        for y in minimum_point.0..=maximum_point.0 {
            for x in minimum_point.1..=maximum_point.1 {
                if let Some((_, count)) = self.point_counter.get_key_value(&Point(x, y)) {
                    output += &count.to_string();
                } else {
                    output += ".";
                }
                if x == maximum_point.1 {
                    output += "\n";
                }
            }
        }

        output
    }
}

pub struct BaseMap {
    point_counter: IndexMap<Point, u64>,
}

impl BaseMap {
    fn init() -> Self {
        Self {
            point_counter: IndexMap::new(),
        }
    }

    fn update_counter(&mut self, input_str: &str, with_diagonals: bool) {
        let line = Line::from(
            input_str
                .trim()
                .split(" -> ")
                .collect_tuple::<(&str, &str)>()
                .expect("Cannot split by ->"),
        );
        let points = line.as_points();
        if !with_diagonals {
            if let Line::Diagonal(_) = line {
                return;
            }
        }
        for point in points {
            self.point_counter
                .entry(point)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
    }

    fn find_intersections_with_sample(&mut self, with_diagonals: bool) {
        for line in SAMPLE_INPUT {
            self.update_counter(line, with_diagonals);
        }
    }

    fn find_intersections(&mut self, with_diagonals: bool) {
        let file = std::fs::File::open("res/day_5.txt").expect("Cannot open file");
        let mut file_buffer = std::io::BufReader::new(file);
        let mut line_buffer = String::new();

        loop {
            match file_buffer.read_line(&mut line_buffer) {
                Err(_) | Ok(0) => break,
                Ok(_) => self.update_counter(&line_buffer, with_diagonals),
            }
            line_buffer.clear();
        }
    }
}
