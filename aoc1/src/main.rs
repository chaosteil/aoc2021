use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./input/input")?;
    let lines: Vec<_> = io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
    Ok(())
}

fn part_one(lines: &[i32]) -> i32 {
    let mut last = lines.first().unwrap();
    let mut current = 0;
    for line in lines.iter().skip(1) {
        if line > last {
            current += 1;
        }
        last = line;
    }
    current
}

fn part_two(lines: &[i32]) -> i32 {
    let mut last = None;
    let mut inc = 0;
    for measurements in lines.windows(3) {
        let sum: i32 = measurements.iter().sum();
        if let Some(s) = last {
            if sum > s {
                inc += 1;
            }
        }
        last = Some(sum)
    }
    inc
}
