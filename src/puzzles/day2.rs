use std::str::FromStr;

use anyhow::{Context, Error};
use aoc_2025::util::parse_csv;
use itertools::Itertools;

fn count_digits(n: i64) -> i32 {
    let mut count = 1;
    let mut exp = 10;
    while exp < n {
        exp *= 10;
        count += 1;
    }
    dbg!(&n, &count);
    count
}

fn is_id_invalid_first(id: i64) -> bool {
    let s = id.to_string();
    let (p1, p2) = s.split_at(s.len() / 2);
    p1 == p2
}

fn is_id_invalid_second(id: i64) -> bool {
    let s = id.to_string();
    for i in 1..=s.len() / 2 {
        if s.len().is_multiple_of(i) {
            let chars = s.chars().collect::<Vec<char>>();
            let mut slices = chars.chunks(i);
            if slices.all_equal() {
                return true;
            }
        }
    }
    false
}

#[allow(dead_code)]
fn is_id_invalid_third(id: i64) -> bool {
    let max_chunk = count_digits(id) / 2;

    for _i in 1..=max_chunk {
        #[allow(unused_variables)]
        let num = id;
    }
    false
}

struct MyRange {
    start: i64,
    end: i64,
}

impl FromStr for MyRange {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split('-');
        Ok(Self {
            start: parts
                .next()
                .ok_or_else(|| anyhow::anyhow!("error getting first part"))?
                .parse()
                .with_context(|| anyhow::anyhow!("parsing {s}"))?,
            end: parts
                .next()
                .ok_or_else(|| anyhow::anyhow!("error getting second part"))?
                .parse()
                .with_context(|| anyhow::anyhow!("parsing {s}"))?,
        })
    }
}

impl MyRange {
    fn invalid_sum_first(&self) -> i64 {
        (self.start..=self.end)
            .filter(|n| is_id_invalid_first(*n))
            .sum()
    }
    fn invalid_sum_second(&self) -> i64 {
        (self.start..=self.end)
            .filter(|n| is_id_invalid_second(*n))
            .sum()
    }
}

#[allow(clippy::unnecessary_wraps)]
pub fn solve(data: &str) -> Result<(String, String), Error> {
    println!("Text input: {data}");

    let ranges = parse_csv::<MyRange>(data)?;

    let tests = vec![11, 12, 121_212, 44444];

    for ele in tests {
        is_id_invalid_second(ele);
    }

    Ok((
        ranges
            .iter()
            .map(MyRange::invalid_sum_first)
            .sum::<i64>()
            .to_string(),
        ranges
            .iter()
            .map(MyRange::invalid_sum_second)
            .sum::<i64>()
            .to_string(),
    ))
}
