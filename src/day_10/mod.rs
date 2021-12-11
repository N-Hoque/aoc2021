use std::collections::VecDeque;

use crate::read_to_string;

const SAMPLE_INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

fn find_illegal_bracket(line: &str) -> Option<char> {
    let mut bracket_stack = VecDeque::new();

    let line_length = line.len();
    for (idx, ch) in line.chars().enumerate() {
        if idx == line_length - 1 || ch == '[' || ch == '{' || ch == '(' || ch == '<' {
            bracket_stack.push_back(ch);
        } else {
            match (ch, bracket_stack.pop_back()) {
                (']', Some(b)) if b != '[' => {
                    return Some(ch);
                }
                ('}', Some(b)) if b != '{' => {
                    return Some(ch);
                }
                (')', Some(b)) if b != '(' => {
                    return Some(ch);
                }
                ('>', Some(b)) if b != '<' => {
                    return Some(ch);
                }
                _ => continue,
            }
        }
    }

    None
}

fn get_illegal_bracket_score(bracket: char) -> u64 {
    match bracket {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        ' ' => 0,
        _ => panic!("Invalid bracket given: {}", bracket),
    }
}

pub fn solve_sample() -> u64 {
    let total_score = SAMPLE_INPUT
        .lines()
        .filter_map(|l| find_illegal_bracket(l).map(get_illegal_bracket_score))
        .sum();

    println!("Sample 1: {}", total_score);

    total_score
}

pub fn solve_sample_2() -> u64 {
    0
}

pub fn part_1() -> u64 {
    let data = read_to_string("res/day_10.txt");

    let total_score = data
        .lines()
        .filter_map(|l| find_illegal_bracket(l).map(get_illegal_bracket_score))
        .sum();

    println!("Part 1: {}", total_score);

    total_score
}

pub fn part_2() -> u64 {
    0
}

#[cfg(test)]
mod tests;
