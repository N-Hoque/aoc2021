use super::{part_1, part_2, solve_sample, solve_sample_2};

#[test]
fn test_solve_sample() {
    assert_eq!(26397, solve_sample());
}

#[test]
fn test_solve_sample_2() {
    assert_eq!(288957, solve_sample_2());
}

#[test]
fn test_part_1() {
    assert_eq!(392139, part_1());
}

#[test]
fn test_part_2() {
    assert_eq!(4001832844, part_2());
}
