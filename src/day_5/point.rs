#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point(pub u64, pub u64);

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
