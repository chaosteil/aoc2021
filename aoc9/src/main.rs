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

fn part_one(input: &[Vec<u32>]) -> usize {
    let mut value = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if [(0, -1), (-1, 0), (1, 0), (0, 1)]
                .iter()
                .map(|(dx, dy)| {
                    let x = x as i32 + dx;
                    let y = y as i32 + dy;
                    if x < 0 || y < 0 || x >= input[0].len() as i32 || y >= input.len() as i32 {
                        return 9;
                    }
                    input[y as usize][x as usize]
                })
                .all(|i| i > input[y][x])
            {
                value += input[y][x] as usize + 1;
            }
        }
    }
    value
}
fn part_two(input: &[Vec<u32>]) -> u32 {
    let mut visited = HashSet::new();
    let mut basins = vec![];
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if let Some(size) = basin(input, (x, y), &mut visited) {
                basins.push(size);
            }
        }
    }
    basins.sort_unstable();
    basins.iter().rev().take(3).product()
}

fn basin(
    input: &[Vec<u32>],
    pos: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
) -> Option<u32> {
    if visited.contains(&pos) {
        return None;
    }
    visited.insert(pos);
    if input[pos.1][pos.0] == 9 {
        return None;
    }

    Some(
        [(0, -1), (-1, 0), (1, 0), (0, 1)]
            .iter()
            .filter_map(|(dx, dy)| {
                let x = pos.0 as i32 + dx;
                let y = pos.1 as i32 + dy;
                if x < 0 || y < 0 || x >= input[0].len() as i32 || y >= input.len() as i32 {
                    return None;
                }
                basin(input, (x as usize, y as usize), visited).or(Some(0))
            })
            .sum::<u32>()
            + 1,
    )
}
