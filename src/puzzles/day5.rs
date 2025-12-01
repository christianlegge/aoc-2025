use std::str::FromStr;

use aoc_2025::util::comma_split_numbers;

#[derive(Debug, Default)]
struct Fishbone {
    segments: Vec<Segment>,
}

#[derive(Debug)]
struct Segment {
    left: Option<i64>,
    spine: i64,
    right: Option<i64>,
}

#[derive(Debug)]
struct Sword {
    identifier: i64,
    fishbone: Fishbone,
}

impl Ord for Sword {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.fishbone.get_quality();
        let b = other.fishbone.get_quality();
        if a != b {
            a.cmp(&b)
        } else {
            let a_iter = self.fishbone.segments.iter();
            let mut b_iter = other.fishbone.segments.iter();
            for x in a_iter {
                let y = b_iter.next().unwrap();
                let x_num = x.get_concat_number();
                let y_num = y.get_concat_number();
                if x_num != y_num {
                    return x_num.cmp(&y_num);
                }
            }
            self.identifier.cmp(&other.identifier)
        }
    }
}

impl PartialOrd for Sword {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Sword {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl Eq for Sword {}

impl FromStr for Sword {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(":").collect::<Vec<&str>>();
        let numbers = comma_split_numbers::<i64>(parts[1].to_owned());
        if let Some(id) = parts.first() {
            Ok(Sword {
                identifier: id.parse::<i64>().unwrap(),
                fishbone: Fishbone::from_numbers(numbers.into_iter()),
            })
        } else {
            panic!("error constructing sword: {}", s);
        }
    }
}

impl Segment {
    pub fn get_concat_number(&self) -> i64 {
        let mut number = String::default();
        if let Some(n) = self.left {
            number += &n.to_string();
        }
        number += &self.spine.to_string();
        if let Some(n) = self.right {
            number += &n.to_string();
        }
        number.parse().unwrap()
    }
}

impl Fishbone {
    pub fn from_numbers<T>(nums: T) -> Self
    where
        T: Iterator<Item = i64>,
    {
        let mut fishbone = Fishbone::default();
        for num in nums {
            fishbone.add_number(num);
        }
        fishbone
    }

    pub fn add_number(&mut self, n: i64) {
        for segment in &mut self.segments {
            if n < segment.spine && segment.left.is_none() {
                segment.left = Some(n);
                return;
            } else if n > segment.spine && segment.right.is_none() {
                segment.right = Some(n);
                return;
            }
        }
        self.segments.push(Segment {
            spine: n,
            left: None,
            right: None,
        });
    }

    pub fn get_quality(&self) -> i64 {
        let mut quality = String::default();
        for segment in &self.segments {
            quality += &segment.spine.to_string();
        }
        quality.parse().unwrap()
    }
}

pub fn solve(data: String) {
    println!("Text input: {}", data);
    let parts = data.split(":").collect::<Vec<&str>>();
    let numbers = comma_split_numbers(parts[1].to_owned());
    let mut fishbone = Fishbone::default();
    for number in numbers {
        fishbone.add_number(number);
    }
    dbg!(&fishbone);
    println!("quality: {}", fishbone.get_quality());
}

pub fn solve2(data: String) {
    let mut swords = data
        .split_whitespace()
        .map(|s| Sword::from_str(s).unwrap())
        .collect::<Vec<Sword>>();
    // swords.sort_by(|sword, other| {
    //     sword
    //         .fishbone
    //         .get_quality()
    //         .cmp(&other.fishbone.get_quality())
    // });
    swords.sort();
    dbg!(&swords.first());
    dbg!(&swords.last());
    println!(
        "diff: {}",
        swords.last().unwrap().fishbone.get_quality()
            - swords.first().unwrap().fishbone.get_quality()
    );
}

pub fn solve3(data: String) {
    let mut swords = data
        .split_whitespace()
        .map(|s| Sword::from_str(s).unwrap())
        .collect::<Vec<Sword>>();
    swords.sort();
    let c = swords
        .iter()
        .rev()
        .enumerate()
        .fold(0, |checksum, (i, sword)| {
            (i as i64 + 1) * sword.identifier + checksum
        });
    dbg!(c);
}
