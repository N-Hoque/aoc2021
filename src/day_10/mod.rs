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

fn complete_brackets(line: &str) -> String {
    let mut bracket_stack = VecDeque::new();

    for (idx, ch) in line.chars().enumerate() {
        if ch == '[' || ch == '{' || ch == '(' || ch == '<' {
            bracket_stack.push_back(ch);
        } else {
            match (ch, bracket_stack.pop_back()) {
                (']', Some('[')) => {
                    continue;
                }
                ('}', Some('{')) => {
                    continue;
                }
                (')', Some('(')) => {
                    continue;
                }
                ('>', Some('<')) => {
                    continue;
                }
                (x, y) => panic!("On line {}, char {}: {} {:?}", line, idx, x, y),
            }
        }
    }

    let mut rest = String::new();

    while let Some(b) = bracket_stack.pop_back() {
        match b {
            ']' => bracket_stack.push_front('['),
            '}' => bracket_stack.push_front('{'),
            ')' => bracket_stack.push_front('('),
            '>' => bracket_stack.push_front('<'),
            '[' => rest += "]",
            '{' => rest += "}",
            '(' => rest += ")",
            '<' => rest += ">",
            _ => panic!("Nothing else should enter here"),
        }
    }

    rest
}

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

fn get_legal_bracket_score(bracket: char) -> u64 {
    match bracket {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("Invalid bracket given: {}", bracket),
    }
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
    let mut line_scores = Vec::new();

    for line in SAMPLE_INPUT.lines() {
        let mut total_score = 0;

        let illegal_char = find_illegal_bracket(line);
        if illegal_char.is_none() {
            let rest = complete_brackets(line);
            for ch in rest.chars() {
                total_score = (total_score * 5) + get_legal_bracket_score(ch);
            }
        }

        if total_score != 0 {
            line_scores.push(total_score);
        }
    }

    let total_scores = line_scores.len();

    line_scores.sort_unstable();

    println!("{:?}", line_scores);

    let mid_score = line_scores[total_scores / 2];

    println!("Sample 2: {}", mid_score);

    mid_score
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
    let data = read_to_string("res/day_10.txt");

    let mut line_scores = Vec::new();

    for line in data.lines() {
        let mut total_score = 0;

        let illegal_char = find_illegal_bracket(line);
        if illegal_char.is_none() {
            let rest = complete_brackets(line);
            for ch in rest.chars() {
                total_score = (total_score * 5) + get_legal_bracket_score(ch);
            }
        }

        if total_score != 0 {
            line_scores.push(total_score);
        }
    }

    let total_scores = line_scores.len();

    line_scores.sort_unstable();

    let mid_score = line_scores[total_scores / 2];

    println!("Part 2: {}", mid_score);

    mid_score
}

#[cfg(test)]
mod tests;
