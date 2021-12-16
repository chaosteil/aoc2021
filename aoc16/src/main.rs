use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
};

struct Transmission {
    bits: Vec<u8>,
    pos: usize,
}

struct Packet {
    version: usize,
    payload: Payload,
}
enum Payload {
    Literal(usize),
    Operator { op: usize, packets: Vec<Packet> },
}

impl Packet {
    fn version_sum(&self) -> usize {
        self.version
            + match &self.payload {
                Payload::Operator { op: _, packets } => {
                    packets.iter().map(|h| h.version_sum()).sum()
                }
                _ => 0,
            }
    }
    fn value(&self) -> usize {
        match &self.payload {
            Payload::Literal(v) => *v,
            Payload::Operator { op, packets } => {
                let mut p = packets.iter().map(|p| p.value());
                match op {
                    0 => p.sum(),
                    1 => p.product(),
                    2 => p.min().unwrap(),
                    3 => p.max().unwrap(),
                    5 => (p.next().unwrap() > p.next().unwrap()) as usize,
                    6 => (p.next().unwrap() < p.next().unwrap()) as usize,
                    7 => (p.next().unwrap() == p.next().unwrap()) as usize,
                    _ => unreachable!(),
                }
            }
        }
    }
}

impl Transmission {
    fn new(bits: &[u8]) -> Self {
        Self {
            bits: bits.to_vec(),
            pos: 0,
        }
    }
    fn read(&mut self) -> Packet {
        let version = self.get(3);
        let op = self.get(3);
        if op == 4 {
            let mut result = 0;
            while self.get(1) == 1 {
                result |= self.get(4);
                result <<= 4;
            }
            result |= self.get(4);
            Packet {
                version,
                payload: Payload::Literal(result),
            }
        } else {
            let ltype = self.get(1);
            Packet {
                version,
                payload: if ltype == 0 {
                    let next = self.get(15) + self.pos;
                    let mut packets = vec![];
                    while self.pos < next {
                        packets.push(self.read());
                    }
                    Payload::Operator { op, packets }
                } else {
                    Payload::Operator {
                        op,
                        packets: (0..self.get(11)).map(|_| self.read()).collect(),
                    }
                },
            }
        }
    }
    fn get(&mut self, num: usize) -> usize {
        let mut result = 0;
        for i in 0..num {
            let pos = self.pos + i;
            result = (result << 1) | (self.bits[pos / 4] as usize >> (3 - (pos % 4))) & 1
        }
        self.pos += num;
        result
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./input/input")?;
    let lines: Vec<u8> = io::BufReader::new(file)
        .lines()
        .find_map(Result::ok)
        .unwrap()
        .chars()
        .map(|c| match c {
            '0'..='9' => c as u8 - b'0',
            'A'..='F' => (c as u8 - b'A') + 0b1010,
            _ => unreachable!(),
        })
        .collect();
    println!("{:?}", part_one(&lines));
    println!("{:?}", part_two(&lines));
    Ok(())
}

fn part_one(input: &[u8]) -> usize {
    let mut t = Transmission::new(input);
    let h = t.read();
    h.version_sum()
}

fn part_two(input: &[u8]) -> usize {
    let mut t = Transmission::new(input);
    let h = t.read();
    h.value()
}
