use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{self, BufRead},
};

type Range = (isize, isize);
type Cube = (Range, Range, Range);

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./input/input")?;
    let lines: Vec<(bool, (Range, Range, Range))> = io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|s| {
            let s = s.split_once(' ').unwrap();
            let toggle = match s.0 {
                "on" => true,
                "off" => false,
                _ => unreachable!(),
            };
            let mut pos = s.1.split(',');
            let (x1, x2) = pos.next().unwrap().split_once("..").unwrap();
            let (y1, y2) = pos.next().unwrap().split_once("..").unwrap();
            let (z1, z2) = pos.next().unwrap().split_once("..").unwrap();
            (
                toggle,
                (
                    (
                        x1[2..].parse::<isize>().unwrap(),
                        x2.parse::<isize>().unwrap(),
                    ),
                    (
                        y1[2..].parse::<isize>().unwrap(),
                        y2.parse::<isize>().unwrap(),
                    ),
                    (
                        z1[2..].parse::<isize>().unwrap(),
                        z2.parse::<isize>().unwrap(),
                    ),
                ),
            )
        })
        .collect();
    println!("{:?}", part_one(&lines));
    println!("{:?}", part_two(&lines));
    Ok(())
}

fn part_one(input: &[(bool, Cube)]) -> usize {
    let mut h = HashSet::<(isize, isize, isize)>::new();
    for (toggle, (x, y, z)) in input.iter().filter(|(_, (x, y, z))| {
        !(x.1 < -50 || x.0 > 50 || y.1 < -50 || y.0 > 50 || z.1 < -50 || z.0 > 50)
    }) {
        let x = x.clamp(&(-50, -50), &(50, 50));
        let y = y.clamp(&(-50, -50), &(50, 50));
        let z = z.clamp(&(-50, -50), &(50, 50));
        for z in z.0..=z.1 {
            for y in y.0..=y.1 {
                for x in x.0..=x.1 {
                    if *toggle {
                        h.insert((x, y, z));
                    } else {
                        h.remove(&(x, y, z));
                    }
                }
            }
        }
    }
    h.len()
}

struct ToggleCube {
    toggle: bool,
    cube: Cube,
}

fn part_two(input: &[(bool, Cube)]) -> usize {
    let mut v = Vec::<ToggleCube>::new();
    for (toggle, cube) in input.iter() {
        let mut add = Vec::new();
        if *toggle {
            add.push(ToggleCube {
                toggle: true,
                cube: *cube,
            });
        }
        for tc in v.iter() {
            if let Some(ic) = intersection(cube, &tc.cube) {
                add.push(ToggleCube {
                    toggle: !tc.toggle,
                    cube: ic,
                });
            }
        }
        v.extend(add);
    }
    v.iter()
        .map(|tc| {
            let sign = if tc.toggle { 1 } else { -1 };
            let (x, y, z) = tc.cube;
            sign * ((x.1 - x.0) as isize + 1)
                * (((y.1 - y.0) as isize) + 1)
                * ((z.1 - z.0) as isize + 1)
        })
        .sum::<isize>() as usize
}

fn intersection(left: &Cube, right: &Cube) -> Option<Cube> {
    let c = (
        (left.0 .0.max(right.0 .0), left.0 .1.min(right.0 .1)),
        (left.1 .0.max(right.1 .0), left.1 .1.min(right.1 .1)),
        (left.2 .0.max(right.2 .0), left.2 .1.min(right.2 .1)),
    );
    if c.0 .0 <= c.0 .1 && c.1 .0 <= c.1 .1 && c.2 .0 <= c.2 .1 {
        Some(c)
    } else {
        None
    }
}
