#![allow(clippy::needless_range_loop)]

use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{self, BufRead},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./input/input")?;
    let lines: Vec<Vec<u32>> = io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|s| s.split("").map(|s| s.to_string()).collect::<Vec<_>>())
        .map(|s| s.iter().filter_map(|v| v.parse::<u32>().ok()).collect())
        .collect();
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
    Ok(())
}

const DIRS: &[(isize, isize)] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn part_one(input: &[Vec<u32>]) -> usize {
    let mut input = input.to_vec();
    let mut total = 0;
    for _ in 0..100 {
        let mut flashes = HashSet::new();
        let mut queue = Vec::new();
        for y in 0..input.len() {
            for x in 0..input[y].len() {
                input[y][x] += 1;
                if input[y][x] > 9 {
                    queue.push((x, y));
                }
            }
        }
        while !queue.is_empty() {
            let (x, y) = queue.pop().unwrap();
            if flashes.contains(&(x, y)) {
                continue;
            }
            input[y][x] += 1;
            if input[y][x] <= 9 {
                continue;
            }
            flashes.insert((x, y));
            for (dx, dy) in DIRS {
                if input
                    .get((y as isize + dy) as usize)
                    .and_then(|i| i.get((x as isize + dx) as usize))
                    .is_some()
                {
                    queue.push(((x as isize + dx) as usize, (y as isize + dy) as usize))
                }
            }
        }
        total += flashes.len();
        for (x, y) in flashes.into_iter() {
            input[y][x] = 0;
        }
    }
    total
}
fn part_two(input: &[Vec<u32>]) -> u32 {
    let (width, height) = (input[0].len(), input.len());
    let mut flashes = HashSet::new();
    let mut input = input.to_vec();
    let mut steps = 0;
    while flashes.len() != width * height {
        flashes.clear();
        steps += 1;
        let mut queue = Vec::new();
        for y in 0..input.len() {
            for x in 0..input[y].len() {
                input[y][x] += 1;
                if input[y][x] > 9 {
                    queue.push((x, y));
                }
            }
        }
        while !queue.is_empty() {
            let (x, y) = queue.pop().unwrap();
            if flashes.contains(&(x, y)) {
                continue;
            }
            input[y][x] += 1;
            if input[y][x] <= 9 {
                continue;
            }
            flashes.insert((x, y));
            for (dx, dy) in DIRS {
                if input
                    .get((y as isize + dy) as usize)
                    .and_then(|i| i.get((x as isize + dx) as usize))
                    .is_some()
                {
                    queue.push(((x as isize + dx) as usize, (y as isize + dy) as usize))
                }
            }
        }
        for &(x, y) in flashes.iter() {
            input[y][x] = 0;
        }
    }
    steps
}
