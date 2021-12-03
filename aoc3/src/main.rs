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
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
    Ok(())
}

fn part_one(input: &[String]) -> u32 {
    let length = input.first().unwrap().len() as u32;
    let input: Vec<u32> = input
        .iter()
        .filter_map(|v| u32::from_str_radix(v, 2).ok())
        .collect();
    let (mut gamma, mut epsilon) = (0, 0);
    for pos in (0..length).rev() {
        let pos = 1 << pos;
        let ones = input.iter().filter(|v| *v & pos > 0).count() > input.len() / 2;
        if ones {
            gamma |= pos;
        } else {
            epsilon |= pos;
        }
    }
    gamma * epsilon
}

fn part_two(input: &[String]) -> u32 {
    let length = input.first().unwrap().len();
    let input: Vec<u32> = input
        .iter()
        .filter_map(|v| u32::from_str_radix(v, 2).ok())
        .collect();
    let (mut most, mut least) = (input.to_vec(), input.to_vec());
    let (mut co2, mut oxy) = (0, 0);
    for pos in (0..length).rev() {
        let pos = 1 << pos;
        let ones = most.iter().filter(|v| *v & pos > 0).count() >= most.len() / 2;
        most = most
            .into_iter()
            .filter(|v| ones == (*v & pos > 0))
            .collect();

        let ones = least.iter().filter(|v| *v & pos > 0).count() < least.len() / 2;
        least = least
            .into_iter()
            .filter(|v| ones == (*v & pos > 0))
            .collect();

        if most.len() == 1 {
            co2 = most[0];
        }
        if least.len() == 1 {
            oxy = least[0];
        }
    }
    co2 * oxy
}
