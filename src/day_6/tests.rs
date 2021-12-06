use super::{part_1, part_2, solve_sample};

#[test]
fn test_solve_sample() {
    assert_eq!(5934, solve_sample(80));
}

#[test]
fn test_part_1() {
    assert_eq!(51034, part_1());
}

#[test]
fn test_part_2() {
    assert_eq!(5434, part_2());
}
