use super::{part_1, part_2, solve_sample, Line, Point};

#[test]
fn start_and_end_are_equal() {
    let line = Line::new(Point(0, 0), Point(0, 0));

    assert_eq!(Point(0, 0), line.get_start());
    assert_eq!(Point(0, 0), line.get_end());
    assert_eq!(Vec::<Point>::new(), line.get_points());
}

#[test]
fn start_and_end_are_separated_by_one_horizontally() {
    let line = Line::new(Point(0, 0), Point(1, 0));

    assert_eq!(Point(0, 0), line.get_start());
    assert_eq!(Point(1, 0), line.get_end());
    assert_eq!(Vec::<Point>::new(), line.get_points());
}

#[test]
fn start_and_end_are_separated_by_one_vertically() {
    let line = Line::new(Point(0, 0), Point(0, 1));

    assert_eq!(Point(0, 0), line.get_start());
    assert_eq!(Point(0, 1), line.get_end());
    assert_eq!(Vec::<Point>::new(), line.get_points());
}

#[test]
fn start_and_end_are_separated_by_one_horizontally_and_flipped() {
    let line = Line::new(Point(1, 0), Point(0, 0));

    assert_eq!(Point(1, 0), line.get_start());
    assert_eq!(Point(0, 0), line.get_end());
    assert_eq!(Vec::<Point>::new(), line.get_points());
}

#[test]
fn start_and_end_are_separated_by_one_vertically_and_flipped() {
    let line = Line::new(Point(0, 1), Point(0, 0));

    assert_eq!(Point(0, 1), line.get_start());
    assert_eq!(Point(0, 0), line.get_end());
    assert_eq!(Vec::<Point>::new(), line.get_points());
}

#[test]
fn start_and_end_are_separated_by_two_horizontally() {
    let line = Line::new(Point(0, 0), Point(2, 0));

    assert_eq!(Point(0, 0), line.get_start());
    assert_eq!(Point(2, 0), line.get_end());
    assert_eq!(vec![Point(1, 0)], line.get_points());
}

#[test]
fn start_and_end_are_separated_by_two_vertically() {
    let line = Line::new(Point(0, 0), Point(0, 2));

    assert_eq!(Point(0, 0), line.get_start());
    assert_eq!(Point(0, 2), line.get_end());
    assert_eq!(vec![Point(0, 1)], line.get_points());
}

#[test]
fn start_and_end_are_separated_by_two_horizontally_and_flipped() {
    let line = Line::new(Point(2, 0), Point(0, 0));

    assert_eq!(Point(2, 0), line.get_start());
    assert_eq!(Point(0, 0), line.get_end());
    assert_eq!(vec![Point(1, 0)], line.get_points());
}

#[test]
fn start_and_end_are_separated_by_two_vertically_and_flipped() {
    let line = Line::new(Point(0, 2), Point(0, 0));

    assert_eq!(Point(0, 2), line.get_start());
    assert_eq!(Point(0, 0), line.get_end());
    assert_eq!(vec![Point(0, 1)], line.get_points());
}

#[test]
fn start_and_end_are_separated_by_one_diagonally() {
    let line = Line::new(Point(0, 0), Point(1, 1));

    assert_eq!(Point(0, 0), line.get_start());
    assert_eq!(Point(1, 1), line.get_end());
    assert_eq!(Vec::<Point>::new(), line.get_points());
}

#[test]
fn start_and_end_are_separated_by_one_diagonally_and_flipped() {
    let line = Line::new(Point(1, 1), Point(0, 0));

    assert_eq!(Point(1, 1), line.get_start());
    assert_eq!(Point(0, 0), line.get_end());
    assert_eq!(Vec::<Point>::new(), line.get_points());
}

#[test]
fn start_and_end_are_separated_by_two_diagonally() {
    let line = Line::new(Point(0, 0), Point(2, 2));

    assert_eq!(Point(0, 0), line.get_start());
    assert_eq!(Point(2, 2), line.get_end());
    assert_eq!(vec![Point(1, 1)], line.get_points());
}

#[test]
fn start_and_end_are_separated_by_two_diagonally_and_flipped() {
    let line = Line::new(Point(2, 2), Point(0, 0));

    assert_eq!(Point(2, 2), line.get_start());
    assert_eq!(Point(0, 0), line.get_end());
    assert_eq!(vec![Point(1, 1)], line.get_points());
}

#[test]
fn test_solve_sample() {
    assert_eq!(5, solve_sample());
}

#[test]
fn test_part_1() {
    assert_eq!(6572, part_1());
}

#[test]
#[ignore = "unimplemented"]
fn test_part_2() {
    assert_eq!(5434, part_2());
}
