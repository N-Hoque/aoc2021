use super::point::Point;

#[derive(Clone)]
pub struct LineData {
    start: Point,
    end: Point,
    points: Vec<Point>,
}

#[derive(Clone)]
pub enum Line {
    Horizontal(LineData),
    Vertical(LineData),
    Diagonal(LineData),
}

impl From<(&str, &str)> for Line {
    fn from(x: (&str, &str)) -> Self {
        Line::new(Point::from(x.0), Point::from(x.1))
    }
}

impl Line {
    pub fn as_points(&self) -> Vec<Point> {
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

    pub fn get_start(&self) -> Point {
        self.get_inner_data().start
    }

    pub fn get_end(&self) -> Point {
        self.get_inner_data().end
    }

    pub fn get_points(&self) -> Vec<Point> {
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
