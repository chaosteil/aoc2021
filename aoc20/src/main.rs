use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{self, BufRead},
};

type Image = HashMap<(isize, isize), i8>;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./input/input")?;
    let lines: Vec<Vec<i8>> = io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|s| {
            s.trim()
                .chars()
                .map(|c| match c {
                    '#' => 1,
                    '.' => 0,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    let algorithm = &lines[0];
    let map = &lines[2..]
        .iter()
        .enumerate()
        .flat_map(|(y, r)| {
            r.iter()
                .enumerate()
                .map(|(x, c)| ((x as isize, y as isize), *c))
                .collect::<Vec<_>>()
        })
        .collect::<Image>();

    println!("{:?}", part_one(algorithm, map));
    println!("{:?}", part_two(algorithm, map));
    Ok(())
}

fn part_one(algorithm: &[i8], image: &Image) -> usize {
    let mut image = image.clone();
    for i in 0..2 {
        image = enhance(infinity(algorithm, i), algorithm, &image);
    }
    image.iter().filter(|(_, v)| **v == 1).count()
}

fn enhance(i: i8, algorithm: &[i8], image: &Image) -> Image {
    let mut output = Image::new();
    let min = image.iter().min_by_key(|(k, _)| *k).unwrap().0;
    let max = image.iter().max_by_key(|(k, _)| *k).unwrap().0;
    for y in (min.1 - 1)..=(max.1 + 1) {
        for x in (min.0 - 1)..=(max.0 + 1) {
            *output.entry((x, y)).or_default() = output_pixel(i, algorithm, image, (x, y))
        }
    }
    output
}

fn output_pixel(i: i8, algorithm: &[i8], image: &Image, pixel: (isize, isize)) -> i8 {
    let (x, y) = pixel;
    let mut value: isize = 0;
    for (x, y) in [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ] {
        value = (value << 1) | (image.get(&(x, y)).unwrap_or(&i) & 1) as isize;
    }
    algorithm[value as usize]
}

fn part_two(algorithm: &[i8], image: &Image) -> usize {
    let mut image = image.clone();
    for i in 0..50 {
        image = enhance(infinity(algorithm, i), algorithm, &image);
    }
    image.iter().filter(|(_, v)| **v == 1).count()
}

fn infinity(algorithm: &[i8], i: i8) -> i8 {
    match algorithm[0] {
        0 => 0,
        1 => i % 2,
        _ => unreachable!(),
    }
}
