use super::{part_1, part_2, solve_sample, solve_sample_2};

#[test]
fn test_solve_sample() {
    assert_eq!(37, solve_sample());
}

#[test]
fn test_solve_sample_2() {
    assert_eq!(168, solve_sample_2());
}

#[test]
fn test_part_1() {
    assert_eq!(337488, part_1());
}

#[test]
fn test_part_2() {
    assert_eq!(89647695, part_2());
}
