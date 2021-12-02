use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./input/input")?;
    let lines: Vec<(String, i32)> = io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|s| {
            let s: Vec<_> = s.split(' ').collect();
            (s[0].to_owned(), s[1].to_owned())
        })
        .filter_map(|s| s.1.parse::<i32>().ok().map(|n| (s.0, n)))
        .collect();
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
    Ok(())
}

fn part_one(instructions: &[(String, i32)]) -> i32 {
    let horizontal: i32 = instructions
        .iter()
        .filter_map(|s| match s.0.as_str() {
            "forward" => Some(s.1),
            _ => None,
        })
        .sum();
    let depth: i32 = instructions
        .iter()
        .filter_map(|s| match s.0.as_str() {
            "down" => Some(s.1),
            "up" => Some(-s.1),
            _ => None,
        })
        .sum();
    horizontal * depth
}

fn part_two(instructions: &[(String, i32)]) -> i32 {
    let (mut aim, mut horizontal, mut depth) = (0, 0, 0);
    for s in instructions {
        match s.0.as_str() {
            "forward" => {
                horizontal += s.1;
                depth += aim * s.1;
            }
            "down" => aim += s.1,
            "up" => aim -= s.1,
            _ => {}
        }
    }
    horizontal * depth
}
