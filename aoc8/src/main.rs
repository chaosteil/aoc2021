use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug)]
struct Data {
    input: Vec<HashSet<char>>,
    output: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./input/input")?;
    let lines: Vec<Data> = io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|s| {
            s.split(" | ")
                .map(|s| s.to_owned())
                .collect::<Vec<String>>()
        })
        .map(|s| Data {
            input: s[0]
                .split(' ')
                .map(|s| s.to_owned().chars().collect())
                .collect(),
            output: s[1].split(' ').map(|s| s.to_owned()).collect(),
        })
        .collect();
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
    Ok(())
}

fn part_one(input: &[Data]) -> u32 {
    input
        .iter()
        .map(|i| {
            i.output.iter().fold(0, |acc, n| {
                acc + match n.len() {
                    2 | 4 | 3 | 7 => 1, // 1, 4, 7, 8
                    _ => 0,
                }
            })
        })
        .sum()
}
fn part_two(input: &[Data]) -> u32 {
    let mut values = 0;
    for i in input {
        let def = HashSet::default();
        let mut nums = vec![&def; 10];
        nums[1] = i.input.iter().find(|s| s.len() == 2).unwrap();
        nums[4] = i.input.iter().find(|s| s.len() == 4).unwrap();
        nums[7] = i.input.iter().find(|s| s.len() == 3).unwrap();
        nums[8] = i.input.iter().find(|s| s.len() == 7).unwrap();

        let fives: Vec<_> = i.input.iter().filter(|s| s.len() == 5).collect();
        nums[2] = fives
            .iter()
            .find(|v| v.intersection(nums[4]).count() == 2)
            .unwrap();
        nums[3] = fives
            .iter()
            .find(|v| v.intersection(nums[2]).count() == 4)
            .unwrap();
        nums[5] = fives
            .iter()
            .find(|v| *v != &nums[2] && *v != &nums[3])
            .unwrap();
        let nine = nums[7].union(nums[5]).map(|v| v.to_owned()).collect();
        nums[9] = &nine;

        let sixes: Vec<_> = i.input.iter().filter(|s| s.len() == 6).collect();
        let zero = sixes
            .iter()
            .find(|v| {
                v.intersection(nums[7])
                    .map(|s| s.to_owned())
                    .collect::<HashSet<char>>()
                    == *nums[7]
                    && **v != nums[9]
            })
            .unwrap();
        nums[0] = zero;
        let six = sixes
            .iter()
            .find(|v| **v != nums[9] && **v != nums[0])
            .unwrap();
        nums[6] = six;

        let result: HashMap<_, _> = nums
            .iter()
            .enumerate()
            .map(|(k, v)| {
                let mut v = v.iter().collect::<Vec<_>>();
                v.sort();
                (String::from_iter(v), k)
            })
            .collect();

        let value = i
            .output
            .iter()
            .map(|s| {
                let mut s: Vec<_> = s.chars().collect();
                s.sort_unstable();
                result[&String::from_iter(s)].to_string()
            })
            .collect::<Vec<String>>()
            .join("")
            .parse::<u32>()
            .unwrap();
        values += value;
    }
    values
}
