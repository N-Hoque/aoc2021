use std::collections::VecDeque;

use itertools::Itertools;

use crate::read_to_string;

const SAMPLE_INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;
        Self { x, y, z }
    }
}

struct HeatmapAnalyser {
    heatmap: Vec<Point>,
}

impl HeatmapAnalyser {
    pub fn new(source: &str) -> Self {
        let data = read_to_string(source);
        Self {
            heatmap: HeatmapAnalyser::generate_heatmap(&data),
        }
    }

    pub fn with_sample() -> Self {
        Self {
            heatmap: HeatmapAnalyser::generate_heatmap(SAMPLE_INPUT),
        }
    }

    fn generate_heatmap(input: &str) -> Vec<Point> {
        let mut points = Vec::new();

        for (idx, line) in input.lines().enumerate() {
            for (jdx, ch) in line.chars().enumerate() {
                points.push(Point::new(
                    idx as i64,
                    jdx as i64,
                    ch.to_digit(10).expect("Cannot convert char to digit") as i64,
                ));
            }
        }

        points
    }

    pub fn calculate_risk(&self) -> u64 {
        let minimal_points = self.find_minimal_points();

        minimal_points.iter().map(|p| (p.z + 1) as u64).sum()
    }

    pub fn calculate_basin_strength(&self) -> u64 {
        let basins = self.find_basins();
        let max_basin_size = basins
            .iter()
            .take(3)
            .map(|b| b.len() as u64)
            .product::<u64>();
        max_basin_size
    }

    fn find_basin(&self, minimal_point: &Point) -> Vec<Point> {
        let mut explored = vec![*minimal_point];

        let initial_neighbors = self
            .find_neighbors(minimal_point)
            .iter()
            .filter(|p| p.z != 9)
            .copied()
            .collect_vec();

        let mut unexplored = VecDeque::from_iter(initial_neighbors.iter().copied());

        while let Some(point_to_search) = unexplored.pop_front() {
            let new_neighbors = self
                .find_neighbors(&point_to_search)
                .iter()
                .filter(|p| p.z != 9)
                .copied()
                .collect_vec();
            explored.push(point_to_search);
            for point in new_neighbors {
                if explored.contains(&point) || unexplored.contains(&point) {
                    continue;
                }
                unexplored.push_back(point);
            }
        }

        let explored = explored.iter().copied().collect_vec();

        explored
    }

    fn find_basins(&self) -> Vec<Vec<Point>> {
        let minimal_points = self.find_minimal_points();
        let mut basins = Vec::new();

        for point in minimal_points {
            basins.push(self.find_basin(&point));
        }

        basins.sort_by_key(|b2| std::cmp::Reverse(b2.len()));

        basins
    }

    fn find_minimal_points(&self) -> Vec<Point> {
        let mut minimal_points = Vec::new();

        for point in &self.heatmap {
            let neighbors = self.find_neighbors(point);
            if neighbors.iter().all(|p| point.z < p.z) {
                minimal_points.push(*point);
            }
        }

        minimal_points
    }

    fn find_neighbors(&self, from: &Point) -> Vec<Point> {
        let mut neighbors = Vec::new();

        for point in &self.heatmap {
            let delta = *point - *from;
            match delta {
                Point { x: 1, y: 0, .. } => neighbors.push(*point),
                Point { x: -1, y: 0, .. } => neighbors.push(*point),
                Point { x: 0, y: 1, .. } => neighbors.push(*point),
                Point { x: 0, y: -1, .. } => neighbors.push(*point),
                _ => continue,
            }
        }

        neighbors
    }
}

fn solve_sample() -> u64 {
    let heatmap_analyser = HeatmapAnalyser::with_sample();
    let risk_level = heatmap_analyser.calculate_risk();

    println!("Sample 1: {}", risk_level);

    risk_level
}

fn solve_sample_2() -> u64 {
    let heatmap_analyser = HeatmapAnalyser::with_sample();
    let max_basin_size = heatmap_analyser.calculate_basin_strength();

    println!("Sample 2: {}", max_basin_size);

    max_basin_size
}

pub fn part_1() -> u64 {
    let heatmap_analyser = HeatmapAnalyser::new("res/day_9.txt");
    let risk_level = heatmap_analyser.calculate_risk();

    println!("Part 1: {}", risk_level);

    risk_level
}

pub fn part_2() -> u64 {
    let heatmap_analyser = HeatmapAnalyser::new("res/day_9.txt");
    let basin_strength = heatmap_analyser.calculate_basin_strength();

    println!("Part 2: {}", basin_strength);

    basin_strength
}

#[cfg(test)]
mod tests;
