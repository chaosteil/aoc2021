use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{self, BufRead},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./input/input")?;
    let lines: Vec<usize> = io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|s| s.split_once(": ").unwrap().1.parse::<usize>().unwrap() - 1)
        .collect();
    println!("{:?}", part_one(&lines));
    println!("{:?}", part_two(&lines));
    Ok(())
}

fn part_one(input: &[usize]) -> usize {
    let mut pos = input.to_vec();
    let mut scores = vec![0, 0];
    let mut die = 0;
    let mut player = 0;
    loop {
        let add = die + 1 + die + 2 + die + 3;
        pos[player] = (pos[player] + add) % 10;
        scores[player] += pos[player] + 1;
        die += 3;
        if scores[player] >= 1000 {
            break;
        }
        player = (player + 1) % 2;
    }
    scores.iter().min().unwrap() * die
}
fn part_two(input: &[usize]) -> u64 {
    let mut cache = HashMap::new();
    let (l, r) = roll(
        0,
        (input[0] as u64, input[1] as u64),
        (0, 0),
        None,
        &mut cache,
    );
    l.max(r)
}

type Positions = (u64, u64);
type Scores = (u64, u64);

fn roll(
    mut player: usize,
    mut pos: Positions,
    mut scores: Scores,
    die: Option<u64>,
    cache: &mut HashMap<(usize, Positions, Scores), Scores>,
) -> Scores {
    if let Some(die) = die {
        if player == 0 {
            pos.0 = (pos.0 + die) % 10;
            scores.0 += pos.0 + 1;
            if scores.0 >= 21 {
                return (1, 0);
            }
        } else {
            pos.1 = (pos.1 + die) % 10;
            scores.1 += pos.1 + 1;
            if scores.1 >= 21 {
                return (0, 1);
            }
        }
        player = (player + 1) % 2;
    }
    if let Some(entry) = cache.get(&(player, pos, scores)) {
        return *entry;
    }
    let (mut left, mut right) = (0, 0);
    for a in 1..=3 {
        for b in 1..=3 {
            for c in 1..=3 {
                let (l, r) = roll(player, pos, scores, Some(a + b + c), cache);
                left += l;
                right += r;
            }
        }
    }
    cache.insert((player, pos, scores), (left, right));
    (left, right)
}
