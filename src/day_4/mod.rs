use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone)]
enum BingoNumber {
    Found(u64),
    NotFound(u64),
}

impl Display for BingoNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BingoNumber::NotFound(x) => write!(f, "{}", x),
            BingoNumber::Found(x) => write!(f, "[{}]", x),
        }
    }
}

impl Default for BingoNumber {
    fn default() -> Self {
        BingoNumber::NotFound(0)
    }
}

struct Board {
    data: Vec<Vec<BingoNumber>>,
}

impl Board {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn has_won(&self) -> bool {
        for row in self.data.iter() {
            if row.iter().all(|x| matches!(x, BingoNumber::Found(_))) {
                return true;
            }
        }

        for row in self.transpose() {
            if row.iter().all(|x| matches!(x, BingoNumber::Found(_))) {
                return true;
            }
        }

        false
    }

    pub fn sum_marked(&self) -> u64 {
        let mut sum = 0;
        for row in self.data.iter() {
            for number in row.iter() {
                if let BingoNumber::Found(x) = number {
                    sum += *x;
                }
            }
        }
        sum
    }

    pub fn sum_unmarked(&self) -> u64 {
        let mut sum = 0;
        for row in self.data.iter() {
            for number in row.iter() {
                if let BingoNumber::NotFound(x) = number {
                    sum += *x;
                }
            }
        }
        sum
    }

    pub fn find(&mut self, value: u64) -> bool {
        for row in self.data.iter_mut() {
            for (idx, number) in row.iter().enumerate() {
                if let BingoNumber::NotFound(x) = number {
                    if *x == value {
                        row[idx] = BingoNumber::Found(value);
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn add_row(&mut self, row: Vec<BingoNumber>) {
        self.data.push(row);
    }

    pub fn transpose(&self) -> Vec<Vec<BingoNumber>> {
        let mut transpose = vec![vec![BingoNumber::default(); 5]; 5];

        for (row_idx, row) in self.data.iter().enumerate() {
            for (col_idx, data) in row.iter().enumerate() {
                transpose[col_idx][row_idx] = *data;
            }
        }

        transpose
    }

    pub fn transpose_mut(&mut self) {
        self.data = self.transpose();
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        for row in self.data.iter() {
            output += &row.iter().map(|x| x.to_string()).join(", ");
            output += "\n"
        }

        write!(f, "{}", output)
    }
}

struct Bingo {
    pool: Vec<u64>,
    boards: Vec<Board>,
}

impl Bingo {
    pub fn new() -> Self {
        let file = std::fs::File::open("res/day_4.txt").expect("Cannot open file");

        let mut file_buffer = BufReader::new(file);
        let mut line_buffer = String::new();
        file_buffer
            .read_line(&mut line_buffer)
            .expect("Cannot read line");

        let pool = line_buffer
            .split(',')
            .filter_map(|x| x.parse::<u64>().ok())
            .collect();

        file_buffer
            .read_line(&mut line_buffer)
            .expect("Cannot read line");
        line_buffer.clear();

        let mut boards = Vec::new();

        let mut current_board = Board::new();

        while let Ok(x) = file_buffer.read_line(&mut line_buffer) {
            if line_buffer.contains(' ') {
                let new_row = line_buffer
                    .split_whitespace()
                    .filter_map(|x| {
                        if let Ok(x) = x.parse::<u64>() {
                            Some(BingoNumber::NotFound(x))
                        } else {
                            None
                        }
                    })
                    .collect();
                current_board.add_row(new_row);
            } else {
                boards.push(current_board);
                current_board = Board::new();
            }
            line_buffer.clear();
            if x == 0 {
                break;
            }
        }

        Self { pool, boards }
    }

    pub fn play(&mut self) -> u64 {
        for number in self.pool.iter() {
            for (idx, board) in self.boards.iter_mut().enumerate() {
                if board.find(*number) {
                    println!("Board {} has {}!", idx, number);
                }
                if board.has_won() {
                    println!("Board {} has won!", idx);
                    println!(
                        "{} x {} = {}",
                        board.sum_unmarked(),
                        number,
                        board.sum_unmarked() * number
                    );
                    return board.sum_unmarked() * number;
                }
            }
        }

        0
    }
}

impl Display for Bingo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let pool_string = self.pool.iter().map(|x| x.to_string()).join(", ");

        let mut boards = String::new();
        for board in self.boards.iter() {
            boards += &board.to_string();
            boards += "\n";
        }

        write!(f, "{}\n\n{}", pool_string, boards)
    }
}

pub fn part_1() -> u64 {
    let mut bingo = Bingo::new();

    bingo.play()
}

pub fn part_2() -> u64 {
    0
}

#[cfg(test)]
mod tests;
