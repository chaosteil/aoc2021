use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./input/input")?;
    let lines: Vec<i32> = io::BufReader::new(file)
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .into_iter()
        .filter_map(|v| v.parse::<i32>().ok())
        .collect();
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
    Ok(())
}

fn part_one(input: &[i32]) -> i32 {
    let mut input = input.to_vec();
    input.sort_unstable();
    let m = input[input.len() / 2];
    input.iter().fold(0, |acc, v| acc + (v - m).abs())
}

fn part_two(input: &[i32]) -> i32 {
    (0..1000)
        .map(|i| {
            input
                .iter()
                .map(|c| {
                    let n = (c - i).abs();
                    (n * (n + 1)) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}
