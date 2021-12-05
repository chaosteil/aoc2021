use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
};

type Point = (i32, i32);
type Line = (Point, Point);

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./input/input")?;
    let lines: Vec<Line> = io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|v| {
            v.replace(" -> ", ",")
                .split(',')
                .filter_map(|v| v.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .map(|nums| ((nums[0], nums[1]), (nums[2], nums[3])))
        .collect();
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
    Ok(())
}

fn part_one(input: &[Line]) -> u32 {
    let input: Vec<Line> = input
        .to_vec()
        .into_iter()
        .filter(|line| line.0 .0 == line.1 .0 || line.0 .1 == line.1 .1)
        .collect();
    let mut board = vec![0; 1000 * 1000];
    for line in input {
        let dir = (
            (line.1 .0 - line.0 .0).signum(),
            (line.1 .1 - line.0 .1).signum(),
        );
        let mut pos = line.0;
        board[pos.1 as usize * 1000 + pos.0 as usize] += 1;
        while pos != line.1 {
            pos.0 += dir.0;
            pos.1 += dir.1;
            board[pos.1 as usize * 1000 + pos.0 as usize] += 1;
        }
    }
    board.iter().filter(|v| **v > 1).count() as u32
}

fn part_two(input: &[Line]) -> u32 {
    let mut board = vec![0; 1000 * 1000];
    for line in input {
        let dir = (
            (line.1 .0 - line.0 .0).signum(),
            (line.1 .1 - line.0 .1).signum(),
        );
        let mut pos = line.0;
        board[pos.1 as usize * 1000 + pos.0 as usize] += 1;
        while pos != line.1 {
            pos.0 += dir.0;
            pos.1 += dir.1;
            board[pos.1 as usize * 1000 + pos.0 as usize] += 1;
        }
    }
    board.iter().filter(|v| **v > 1).count() as u32
}
