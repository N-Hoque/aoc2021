use super::{part_1, part_2, solve_sample};

#[test]
fn test_solve_sample() {
    assert_eq!(5934, solve_sample(80));
}

#[test]
fn test_solve_sample_2() {
    assert_eq!(26984457539, solve_sample(256));
}

#[test]
fn test_part_1() {
    assert_eq!(360268, part_1());
}

#[test]
fn test_part_2() {
    assert_eq!(1632146183902, part_2());
}
