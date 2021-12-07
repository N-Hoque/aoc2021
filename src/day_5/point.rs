use std::fmt::Display;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point(pub u64, pub u64);

impl From<&str> for Point {
    fn from(x: &str) -> Self {
        x.split(',')
            .collect_tuple::<(&str, &str)>()
            .map(Point::from)
            .expect("Cannot create point")
    }
}

impl From<(&str, &str)> for Point {
    fn from((x, y): (&str, &str)) -> Self {
        match (x.parse(), y.parse()) {
            (Err(_), _) => panic!("Could not parse left value {}", x),
            (_, Err(_)) => panic!("Could not parse right value {}", y),
            (Ok(x), Ok(y)) => Point(x, y),
        }
    }
}

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

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}
