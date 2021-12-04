use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{self, BufRead},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./input/input")?;
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect();
    let numbers: Vec<i32> = lines
        .first()
        .unwrap()
        .split(',')
        .filter_map(|v| v.parse::<i32>().ok())
        .collect();
    let boards: Vec<Vec<i32>> = lines
        .into_iter()
        .skip(1)
        .collect::<Vec<String>>()
        .chunks(6)
        .map(|v| {
            v.join(" ")
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect()
        })
        .collect();
    println!("{}", part_one(&numbers, &boards));
    println!("{}", part_two(&numbers, &boards));
    Ok(())
}

fn part_one(numbers: &[i32], boards: &[Vec<i32>]) -> i32 {
    for i in 0..numbers.len() {
        let set = HashSet::from_iter(numbers[..=i].iter().cloned());
        for board in boards {
            if let Some(value) = validate_board(&set, board) {
                return numbers[i] * value;
            }
        }
    }
    0
}

fn validate_board(numbers: &HashSet<i32>, board: &[i32]) -> Option<i32> {
    let mut board = board.to_vec();
    for item in &mut board {
        if numbers.contains(item) {
            *item = -1;
        }
    }
    for y in 0..5 {
        let mut row = 0;
        for x in 0..5 {
            row += board[y * 5 + x];
        }
        if row == -5 {
            return Some(board.iter().filter(|v| **v >= 0).sum());
        }
    }
    for x in 0..5 {
        let mut col = 0;
        for y in 0..5 {
            col += board[y * 5 + x];
        }
        if col == -5 {
            return Some(board.iter().filter(|v| **v >= 0).sum());
        }
    }
    None
}

fn part_two(numbers: &[i32], boards: &[Vec<i32>]) -> i32 {
    let mut boards = boards.to_vec();
    for i in 0..numbers.len() {
        let set = HashSet::from_iter(numbers[..=i].iter().cloned());
        let mut drop_boards = HashSet::new();
        for b in 0..boards.len() {
            if let Some(value) = validate_board(&set, &boards[b]) {
                if boards.len() == 1 {
                    return numbers[i] * value;
                }
                drop_boards.insert(b);
            }
        }
        boards = boards
            .into_iter()
            .enumerate()
            .filter(|&(i, _)| !drop_boards.contains(&i))
            .map(|(_, v)| v)
            .collect();
    }
    0
}
