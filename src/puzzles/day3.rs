use std::str::FromStr;

use anyhow::Error;
use aoc_2025::util::parse_lines;
use itertools::Itertools;

struct Bank {
    joltages: Vec<u64>,
}

impl FromStr for Bank {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            joltages: s
                .chars()
                .map(|c| {
                    c.to_digit(10).map_or_else(
                        || Err(anyhow::anyhow!("Invalid digit {c}")),
                        |n| Ok(u64::from(n)),
                    )
                })
                .try_collect()?,
        })
    }
}

impl Bank {
    fn find_max_2(&self) -> u64 {
        let (mut max1, mut max2) = (1, 1);
        let iter = self.joltages.iter().enumerate();
        for (i, &j) in iter {
            if j > max1 && i < self.joltages.len() - 1 {
                max1 = j;
                max2 = 1;
            } else if j > max2 {
                max2 = j;
            }
            if max1 == 9 && max2 == 9 {
                break;
            }
        }
        max1 * 10 + max2
    }

    fn find_max_arb(&self, len: usize) -> u64 {
        if self.joltages.len() <= len {
            return get_total(&self.joltages);
        }
        let mut result: Vec<u64> = vec![];
        let mut iter = self.joltages.iter();
        let mut finding_ith = 1;
        while result.len() < len {
            let leaving = len - finding_ith;
            let candidates = iter.clone().take(iter.clone().count() - leaving);
            let (idx, i) = get_max(candidates);
            result.push(i);
            for _ in 0..=idx {
                iter.next();
            }
            finding_ith += 1;
        }

        get_total(&result)
    }
}

fn get_total<'a>(vec: impl IntoIterator<Item = &'a u64>) -> u64 {
    let mut total = 0;
    for &i in vec {
        total *= 10;
        total += i;
    }
    total
}

fn get_max<'a>(vec: impl IntoIterator<Item = &'a u64>) -> (usize, u64) {
    let mut max = (0, 0);
    for (idx, &i) in vec.into_iter().enumerate() {
        if i > max.1 {
            max = (idx, i);
        }
    }
    max
}

#[allow(clippy::unnecessary_wraps)]
pub fn solve(data: &str) -> Result<(String, String), Error> {
    let banks = parse_lines::<Bank>(data)?;
    Ok((
        banks.iter().map(Bank::find_max_2).sum::<u64>().to_string(),
        banks
            .iter()
            .map(|b| b.find_max_arb(12))
            .sum::<u64>()
            .to_string(),
    ))
}
