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

fn part_one(input: &[i32]) -> u32 {
    let mut fish = Vec::from(input);
    for _ in 0..80 {
        let mut spawn = 0;
        for f in &mut fish {
            *f -= 1;
            if *f < 0 {
                *f = 6;
                spawn += 1;
            }
        }
        for _ in 0..spawn {
            fish.push(8)
        }
    }
    fish.len() as u32
}
fn part_two(input: &[i32]) -> u64 {
    let mut buckets = vec![0; 9];
    for f in input {
        buckets[*f as usize] += 1;
    }
    for _ in 0..256 {
        buckets.rotate_left(1);
        buckets[6] += buckets[8];
    }
    buckets.iter().sum()
}
