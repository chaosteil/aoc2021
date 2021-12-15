use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    error::Error,
    fs::File,
    io::{self, BufRead},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./input/input")?;
    let lines: Vec<Vec<u8>> = io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|i| i.bytes().map(|v| (v - b'0')).collect())
        .collect();

    println!("{:?}", part_one(&lines));
    println!("{:?}", part_two(&lines));
    Ok(())
}

fn part_one(input: &[Vec<u8>]) -> usize {
    find_score(input)
}
fn part_two(input: &[Vec<u8>]) -> usize {
    let wrap = |v, i| {
        let v = v + i;
        if v >= 10 {
            v - 9
        } else {
            v
        }
    };
    let input: Vec<Vec<_>> = input
        .iter()
        .map(|row| {
            (0..5)
                .map(|i| row.iter().map(move |v| wrap(v, i)))
                .flatten()
                .collect()
        })
        .collect();
    let input: Vec<_> = (0..5)
        .map(|i| {
            input
                .iter()
                .map(move |v| v.iter().map(|v| wrap(v, i)).collect::<Vec<_>>())
        })
        .flatten()
        .collect();
    find_score(&input)
}

#[derive(PartialEq, Eq, Ord, PartialOrd)]
struct Entry {
    value: usize,

    pos: (usize, usize),
}

fn find_score(input: &[Vec<u8>]) -> usize {
    let mut prev = HashMap::new();
    prev.entry((0, 0)).or_insert(0);
    let mut walk = BinaryHeap::new();
    walk.push(Reverse(Entry {
        pos: (0, 0),
        value: 0,
    }));
    let last = (input[0].len() - 1, input.len() - 1);
    loop {
        let min = walk.pop().unwrap().0;
        if min.pos == last {
            break;
        }
        for neighbor in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let (x, y) = (
                (min.pos.0 as i32 + neighbor.0) as usize,
                (min.pos.1 as i32 + neighbor.1) as usize,
            );
            if prev.get(&(x, y)).is_some() {
                continue;
            }
            let value = match input.get(y).and_then(|p| p.get(x)) {
                Some(v) => *v as usize,
                None => continue,
            };
            let entry = Entry {
                pos: (x, y),
                value: min.value + value,
            };
            walk.push(Reverse(entry));
            prev.entry((x, y)).or_insert(min.value + value);
        }
    }
    prev[&last]
}
