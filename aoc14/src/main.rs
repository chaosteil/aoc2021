use std::{
    collections::HashMap,
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
    let template = lines[0].clone();
    let lines = lines[2..lines.len()]
        .iter()
        .map(|x| x.split_once(" -> ").unwrap())
        .map(|(c, v)| ((c.as_bytes()[0] as char, c.as_bytes()[1] as char), v))
        .collect::<HashMap<_, _>>();
    println!("{:?}", part_one(&template, &lines));
    println!("{:?}", part_two(&template, &lines));
    Ok(())
}

fn part_one(template: &str, input: &HashMap<(char, char), &str>) -> usize {
    find_polymer(template, input, 10)
}
fn part_two(template: &str, input: &HashMap<(char, char), &str>) -> usize {
    find_polymer(template, input, 40)
}

// ABC
// AB -> A
// BC -> A
// ABC -> AABAC
//
// AB 1 -> AA 1 AB 1
// BC 1 -> BA 1 AC 1

fn find_polymer(template: &str, input: &HashMap<(char, char), &str>, steps: usize) -> usize {
    let mut counts = HashMap::new();
    for i in template.chars().collect::<Vec<_>>().windows(2) {
        *counts.entry((i[0], i[1])).or_default() += 1;
    }
    for _ in 0..steps {
        let mut current = HashMap::<(char, char), usize>::new();
        for (key, c) in counts.iter() {
            if let Some(letter) = input.get(key) {
                let letter = letter.as_bytes()[0] as char;
                *current.entry((key.0, letter)).or_default() += c;
                *current.entry((letter, key.1)).or_default() += c;
            }
        }
        counts = current;
    }
    let mut counts = counts
        .iter()
        .fold(HashMap::<char, usize>::new(), |mut m, (k, v)| {
            *m.entry(k.1).or_default() += v;
            m
        });
    *counts.entry(template.as_bytes()[0] as char).or_default() += 1;
    let max = counts.iter().max_by_key(|(_, i)| *i).unwrap().1;
    let min = counts.iter().min_by_key(|(_, i)| *i).unwrap().1;
    max - min
}
