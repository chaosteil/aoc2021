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

fn pair(c: char) -> char {
    match c {
        '(' => ')',
        ')' => '(',
        '[' => ']',
        ']' => '[',
        '{' => '}',
        '}' => '{',
        '<' => '>',
        '>' => '<',
        _ => panic!(),
    }
}

fn part_one(input: &[String]) -> u32 {
    let mut score = 0;
    for i in input {
        let mut stack = Vec::new();
        for c in i.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                _ => {
                    if pair(c) != stack.pop().unwrap() {
                        score += match c {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => panic!(),
                        };
                        break;
                    }
                }
            }
        }
    }
    score
}
fn part_two(input: &[String]) -> u64 {
    let mut scores = Vec::new();
    'lines: for i in input {
        let mut stack = Vec::new();
        for c in i.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                _ => {
                    if pair(c) == *stack.last().unwrap() {
                        stack.pop();
                    } else {
                        continue 'lines;
                    }
                }
            }
        }
        scores.push(stack.iter().rev().fold(0, |acc, c| {
            (acc * 5)
                + match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => panic!(),
                }
        }))
    }
    scores.sort_unstable();
    scores[scores.len() / 2]
}
