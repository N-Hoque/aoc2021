use crate::day_5::part_2;

use super::{part_1, solve_sample, solve_sample_2};

#[cfg(test)]
mod line {
    use crate::day_5::{line::Line, point::Point};

    #[test]
    #[ignore = "day 5 testing only"]
    fn start_and_end_are_equal() {
        let line = Line::new(Point(0, 0), Point(0, 0));

        assert_eq!(Point(0, 0), line.get_start());
        assert_eq!(Point(0, 0), line.get_end());
        assert_eq!(Vec::<Point>::new(), line.get_points());
    }

    #[test]
    #[ignore = "day 5 testing only"]
    fn start_and_end_are_separated_by_one_horizontally() {
        let line = Line::new(Point(0, 0), Point(1, 0));

        assert_eq!(Point(0, 0), line.get_start());
        assert_eq!(Point(1, 0), line.get_end());
        assert_eq!(Vec::<Point>::new(), line.get_points());
    }

    #[test]
    #[ignore = "day 5 testing only"]
    fn start_and_end_are_separated_by_one_vertically() {
        let line = Line::new(Point(0, 0), Point(0, 1));

        assert_eq!(Point(0, 0), line.get_start());
        assert_eq!(Point(0, 1), line.get_end());
        assert_eq!(Vec::<Point>::new(), line.get_points());
    }

    #[test]
    #[ignore = "day 5 testing only"]
    fn start_and_end_are_separated_by_one_horizontally_and_flipped() {
        let line = Line::new(Point(1, 0), Point(0, 0));

        assert_eq!(Point(1, 0), line.get_start());
        assert_eq!(Point(0, 0), line.get_end());
        assert_eq!(Vec::<Point>::new(), line.get_points());
    }

    #[test]
    #[ignore = "day 5 testing only"]
    fn start_and_end_are_separated_by_one_vertically_and_flipped() {
        let line = Line::new(Point(0, 1), Point(0, 0));

        assert_eq!(Point(0, 1), line.get_start());
        assert_eq!(Point(0, 0), line.get_end());
        assert_eq!(Vec::<Point>::new(), line.get_points());
    }

    #[test]
    #[ignore = "day 5 testing only"]
    fn start_and_end_are_separated_by_two_horizontally() {
        let line = Line::new(Point(0, 0), Point(2, 0));

        assert_eq!(Point(0, 0), line.get_start());
        assert_eq!(Point(2, 0), line.get_end());
        assert_eq!(vec![Point(1, 0)], line.get_points());
    }

    #[test]
    #[ignore = "day 5 testing only"]
    fn start_and_end_are_separated_by_two_vertically() {
        let line = Line::new(Point(0, 0), Point(0, 2));

        assert_eq!(Point(0, 0), line.get_start());
        assert_eq!(Point(0, 2), line.get_end());
        assert_eq!(vec![Point(0, 1)], line.get_points());
    }

    #[test]
    #[ignore = "day 5 testing only"]
    fn start_and_end_are_separated_by_two_horizontally_and_flipped() {
        let line = Line::new(Point(2, 0), Point(0, 0));

        assert_eq!(Point(2, 0), line.get_start());
        assert_eq!(Point(0, 0), line.get_end());
        assert_eq!(vec![Point(1, 0)], line.get_points());
    }

    #[test]
    #[ignore = "day 5 testing only"]
    fn start_and_end_are_separated_by_two_vertically_and_flipped() {
        let line = Line::new(Point(0, 2), Point(0, 0));

        assert_eq!(Point(0, 2), line.get_start());
        assert_eq!(Point(0, 0), line.get_end());
        assert_eq!(vec![Point(0, 1)], line.get_points());
    }

    #[test]
    #[ignore = "day 5 testing only"]
    fn start_and_end_are_separated_by_one_diagonally() {
        let line = Line::new(Point(0, 0), Point(1, 1));

        assert_eq!(Point(0, 0), line.get_start());
        assert_eq!(Point(1, 1), line.get_end());
        assert_eq!(Vec::<Point>::new(), line.get_points());
    }

    #[test]
    #[ignore = "day 5 testing only"]
    fn start_and_end_are_separated_by_one_diagonally_and_flipped() {
        let line = Line::new(Point(1, 1), Point(0, 0));

        assert_eq!(Point(1, 1), line.get_start());
        assert_eq!(Point(0, 0), line.get_end());
        assert_eq!(Vec::<Point>::new(), line.get_points());
    }

    #[test]
    #[ignore = "day 5 testing only"]
    fn start_and_end_are_separated_by_two_diagonally() {
        let line = Line::new(Point(0, 0), Point(2, 2));

        assert_eq!(Point(0, 0), line.get_start());
        assert_eq!(Point(2, 2), line.get_end());
        assert_eq!(vec![Point(1, 1)], line.get_points());
    }

    #[test]
    #[ignore = "day 5 testing only"]
    fn start_and_end_are_separated_by_two_diagonally_and_flipped() {
        let line = Line::new(Point(2, 2), Point(0, 0));

        assert_eq!(Point(2, 2), line.get_start());
        assert_eq!(Point(0, 0), line.get_end());
        assert_eq!(vec![Point(1, 1)], line.get_points());
    }

    #[test]
    #[ignore = "day 5 testing only"]
    fn start_and_end_are_separated_by_two_cross_diagonally() {
        let line = Line::new(Point(0, 2), Point(2, 0));

        assert_eq!(Point(0, 2), line.get_start());
        assert_eq!(Point(2, 0), line.get_end());
        assert_eq!(vec![Point(1, 1)], line.get_points());
    }

    #[test]
    #[ignore = "day 5 testing only"]
    fn start_and_end_are_separated_by_two_cross_diagonally_and_flipped() {
        let line = Line::new(Point(2, 0), Point(0, 2));

        assert_eq!(Point(2, 0), line.get_start());
        assert_eq!(Point(0, 2), line.get_end());
        assert_eq!(vec![Point(1, 1)], line.get_points());
    }

    #[test]
    #[ignore = "day 5 testing only"]
    fn start_and_end_are_separated_by_three_diagonally() {
        let line = Line::new(Point(0, 0), Point(3, 3));

        assert_eq!(Point(0, 0), line.get_start());
        assert_eq!(Point(3, 3), line.get_end());
        assert_eq!(vec![Point(1, 1), Point(2, 2)], line.get_points());
    }

    #[test]
    #[ignore = "day 5 testing only"]
    fn start_and_end_are_separated_by_three_diagonally_and_flipped() {
        let line = Line::new(Point(3, 3), Point(0, 0));

        assert_eq!(Point(3, 3), line.get_start());
        assert_eq!(Point(0, 0), line.get_end());
        assert_eq!(vec![Point(2, 2), Point(1, 1)], line.get_points());
    }

    #[test]
    #[ignore = "day 5 testing only"]
    fn start_and_end_are_separated_by_three_cross_diagonally() {
        let line = Line::new(Point(0, 3), Point(3, 0));

        assert_eq!(Point(0, 3), line.get_start());
        assert_eq!(Point(3, 0), line.get_end());
        assert_eq!(vec![Point(1, 2), Point(2, 1)], line.get_points());
    }

    #[test]
    #[ignore = "day 5 testing only"]
    fn start_and_end_are_separated_by_three_cross_diagonally_and_flipped() {
        let line = Line::new(Point(3, 0), Point(0, 3));

        assert_eq!(Point(3, 0), line.get_start());
        assert_eq!(Point(0, 3), line.get_end());
        assert_eq!(vec![Point(2, 1), Point(1, 2)], line.get_points());
    }

    #[test]
    #[ignore = "day 5 testing only"]
    fn start_and_end_are_separated_by_four_cross_diagonally_and_not_from_origin() {
        let line = Line::new(Point(2, 0), Point(6, 4));

        assert_eq!(Point(2, 0), line.get_start());
        assert_eq!(Point(6, 4), line.get_end());
        assert_eq!(
            vec![Point(3, 1), Point(4, 2), Point(5, 3)],
            line.get_points()
        );
    }

    #[test]
    #[ignore = "day 5 testing only"]
    fn start_and_end_are_separated_by_three_cross_diagonally_and_flipped_and_not_from_origin() {
        let line = Line::new(Point(6, 4), Point(2, 0));

        assert_eq!(Point(6, 4), line.get_start());
        assert_eq!(Point(2, 0), line.get_end());
        assert_eq!(
            vec![Point(5, 3), Point(4, 2), Point(3, 1)],
            line.get_points()
        );
    }
}

#[test]
fn test_solve_sample() {
    assert_eq!(5, solve_sample());
}

#[test]
fn test_solve_sample_2() {
    assert_eq!(12, solve_sample_2());
}

#[test]
fn test_part_1() {
    assert_eq!(6572, part_1());
}

#[test]
fn test_part_2() {
    assert_eq!(21466, part_2());
}
