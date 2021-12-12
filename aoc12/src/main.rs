use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{self, BufRead},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./input/input")?;
    let lines: Vec<(String, String)> = io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|x| x.split('-').map(|x| x.to_owned()).collect::<Vec<String>>())
        .map(|x| (x[0].to_owned(), x[1].to_owned()))
        .collect();
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
    Ok(())
}

fn prepare(input: &[(String, String)]) -> HashMap<String, Vec<String>> {
    let mut paths = HashMap::new();
    for i in input {
        paths
            .entry(i.0.clone())
            .or_insert_with(Vec::new)
            .push(i.1.clone());
        paths
            .entry(i.1.clone())
            .or_insert_with(Vec::new)
            .push(i.0.clone());
    }
    paths
}

fn part_one(input: &[(String, String)]) -> usize {
    let paths = prepare(input);
    find_path(&paths, &["start".to_string()], true)
}

fn part_two(input: &[(String, String)]) -> usize {
    let paths = prepare(input);
    find_path(&paths, &["start".to_string()], false)
}

fn find_path(input: &HashMap<String, Vec<String>>, prev: &[String], twice: bool) -> usize {
    if *prev.last().unwrap() == "end" {
        return 1;
    }
    let next = &input[prev.last().unwrap()];
    next.iter()
        .filter(|x| {
            if *x == "start" {
                false
            } else if twice && x.chars().all(|c| c.is_lowercase()) {
                !prev.contains(x)
            } else {
                true
            }
        })
        .map(|x| {
            let twice = twice || (x.chars().all(|c| c.is_lowercase()) && prev.contains(x));
            let mut prev = prev.to_vec();
            prev.push(x.clone());
            find_path(input, &prev, twice)
        })
        .sum()
}
