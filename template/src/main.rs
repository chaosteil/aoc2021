use std::{
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
    println!("{:?}", part_one(&lines));
    println!("{:?}", part_two(&lines));
    Ok(())
}

fn part_one(input: &[String]) -> u32 {
    0
}
fn part_two(input: &[String]) -> u32 {
    0
}
