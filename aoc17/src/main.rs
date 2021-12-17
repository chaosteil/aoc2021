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
    let aabb: Vec<_> = lines[0]
        .split_terminator(&['=', '.', ','][..])
        .filter_map(|x| x.parse::<i32>().ok())
        .collect::<Vec<i32>>();
    dbg!(&aabb);
    println!("{:?}", part_one(&aabb));
    println!("{:?}", part_two(&aabb));
    Ok(())
}

fn part_one(aabb: &[i32]) -> i32 {
    let y = -aabb[2];
    (y * (y - 1)) / 2
}
fn part_two(aabb: &[i32]) -> i32 {
    let mut count = 0;
    for dy in aabb[2]..=(-aabb[2]) {
        for dx in 0..=aabb[1] {
            let (mut dx, mut dy) = (dx, dy);
            let (mut pos_x, mut pos_y) = (0, 0);
            while pos_x <= aabb[1] && pos_y >= aabb[2] {
                if pos_x >= aabb[0] && pos_y <= aabb[3] {
                    count += 1;
                    break;
                }
                pos_x += dx;
                pos_y += dy;
                dx = (dx - 1).max(0);
                dy -= 1;
            }
        }
    }
    count
}
