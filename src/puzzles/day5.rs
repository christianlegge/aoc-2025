use std::{collections::HashSet, ops::RangeInclusive, str::FromStr};

use anyhow::Error;
use itertools::Itertools;

#[derive(Debug)]
struct IngredientDb {
    fresh: Vec<RangeInclusive<u64>>,
    available: HashSet<u64>,
}

impl FromStr for IngredientDb {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.lines();
        let mut fresh = Vec::new();
        for l in (&mut iter).take_while(|&l| !l.is_empty()) {
            let mut parts = l.split('-').map(str::parse::<u64>);
            match (parts.next(), parts.next()) {
                (Some(a), Some(b)) => fresh.push(a?..=b?),
                _ => return Err(anyhow::anyhow!("Error converting string {s}")),
            }
        }
        iter.next();
        let mut available = HashSet::new();
        for l in iter {
            available.insert(l.parse::<u64>()?);
        }
        Ok(Self { fresh, available })
    }
}

fn sum_ranges(ranges: &Vec<RangeInclusive<u64>>) -> u64 {
    let mut sum = 0;
    for i in ranges {
        sum += i.end() - i.start() + 1;
    }
    sum
}

fn expand_range_ordered(
    range: &RangeInclusive<u64>,
    other: &RangeInclusive<u64>,
) -> Option<RangeInclusive<u64>> {
    if range.end() >= other.start() {
        Some(RangeInclusive::new(
            *range.start(),
            *other.end().max(range.end()),
        ))
    } else {
        None
    }
}

fn merge_overlaps(ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    let mut ranges = ranges;

    loop {
        ranges.sort_by_key(|r| *r.start());
        println!("\nstart: {ranges:?}");
        let mut changed = false;
        let mut new_ranges = Vec::new();

        let mut iter = ranges.iter().peekable();
        while iter.peek().is_some() {
            if let Some(cur) = iter.next() {
                if let Some(&other) = iter.peek() {
                    println!("checking {cur:?} and {other:?}...");
                    if let Some(merged) = expand_range_ordered(cur, other) {
                        println!("merging into {merged:?}");
                        changed = true;
                        new_ranges.push(merged);
                        iter.next();
                    } else {
                        new_ranges.push(cur.clone());
                    }
                } else {
                    new_ranges.push(cur.clone());
                }
            }
        }

        // for rs in ranges.windows(2) {
        //     let a = rs[0].clone();
        //     let b = rs[1].clone();
        //     println!("checking {a:?} and {b:?}...");
        //     if let Some(merged) = expand_range_ordered(&a, &b) {
        //         println!("merging into {merged:?}");
        //         changed = true;
        //         new_ranges.push(merged);
        //     } else {
        //         println!("disjoint");
        //         new_ranges.push(a);
        //         new_ranges.push(b);
        //     }
        // }

        ranges = new_ranges;

        if !changed {
            break;
        }
    }
    ranges
}

impl IngredientDb {
    pub fn reduce_fresh(&self) -> Vec<RangeInclusive<u64>> {
        merge_overlaps(self.fresh.clone())
    }
}

#[allow(clippy::unnecessary_wraps)]
pub fn solve(data: &str) -> Result<(String, String), Error> {
    //     let data = r"3-5
    // 16-20
    // 12-18
    // 10-14
    //
    // 1
    // 5
    // 8
    // 11
    // 17
    // 32";
    println!("Text input: {data}");
    let db = IngredientDb::from_str(data.trim())?;
    let fresh_available = db
        .available
        .iter()
        .filter(|&fruit| db.fresh.iter().any(|range| range.contains(fruit)));

    Ok((
        fresh_available.count().to_string(),
        sum_ranges(&db.reduce_fresh()).to_string(),
    ))
}
