use std::{fmt::Display, str::FromStr};

use anyhow::Error;
use aoc_2025::util::parse_lines;

struct Dial {
    position: i32,
    max: i32,
}

impl Dial {
    const fn make_move(&mut self, m: &Move) {
        self.position = match *m {
            Move::Left(i) => (self.position - i).rem_euclid(self.max),
            Move::Right(i) => (self.position + i).rem_euclid(self.max),
        };
    }

    const fn count_zeroes(&self, m: &Move) -> i32 {
        let (dist_to_zero, move_dist) = match *m {
            Move::Left(i) => (
                if self.position == 0 {
                    self.max
                } else {
                    self.position
                },
                i,
            ),
            Move::Right(i) => (self.max - self.position, i),
        };
        if dist_to_zero > move_dist {
            0
        } else {
            let rem = move_dist - dist_to_zero;
            1 + rem / self.max
        }
    }
}

#[derive(Debug)]
enum Move {
    Left(i32),
    Right(i32),
}

impl FromStr for Move {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some('L') => Ok(Self::Left(s[1..].parse()?)),
            Some('R') => Ok(Self::Right(s[1..].parse()?)),
            _ => Err(anyhow::anyhow!("error parsing string {s}")),
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left(i) => write!(f, "L{i}"),
            Self::Right(i) => write!(f, "R{i}"),
        }
    }
}

pub fn solve(data: &str) -> Result<(String, String), Error> {
    println!("Text input: {data}");
    let mut dial = Dial {
        position: 50,
        max: 100,
    };
    let mut zeroes = 0;
    let mut more_zeroes = 0;
    for m in parse_lines(data)? {
        let passes = dial.count_zeroes(&m);
        dial.make_move(&m);
        if dial.position == 0 {
            zeroes += 1;
        }
        more_zeroes += passes;
        println!(
            "rotated {m} to point at {}, passed {passes} times",
            dial.position,
        );
    }

    Ok((zeroes.to_string(), more_zeroes.to_string()))
}
