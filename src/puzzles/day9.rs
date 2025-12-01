use std::{num::ParseIntError, str::FromStr};

use itertools::Itertools;

#[derive(Debug)]
struct Scale {
    id: i32,
    code: String,
}

fn find_children(scales: &[Scale]) -> i32 {
    let mut sum = 0;
    for i in scales {
        if let Some((p1, p2)) = find_parents(i, scales) {
            sum += find_similarity(&i.code, &p1.code, &p2.code);
        }
    }
    sum
}

fn find_parents<'a>(child: &Scale, scales: &'a [Scale]) -> Option<(&'a Scale, &'a Scale)> {
    for i in scales {
        if i.id == child.id {
            continue;
        }
        let skip2 = i.id;
        for j in scales {
            if j.id == child.id || j.id == skip2 {
                continue;
            }
            if find_child(&child.code, &i.code, &j.code) {
                return Some((i, j));
            }
        }
    }
    None
}

#[derive(Debug)]
enum ScaleParseError {
    InvalidInt,
    FieldError,
}

impl From<ParseIntError> for ScaleParseError {
    fn from(_value: ParseIntError) -> Self {
        ScaleParseError::InvalidInt
    }
}

impl FromStr for Scale {
    type Err = ScaleParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(":");
        Ok(Scale {
            id: parts.next().ok_or(ScaleParseError::FieldError)?.parse()?,
            code: parts.next().ok_or(ScaleParseError::FieldError)?.to_string(),
        })
    }
}

fn find_child(child: &str, parent1: &str, parent2: &str) -> bool {
    for ((c, p1), p2) in child
        .chars()
        .zip_eq(parent1.chars())
        .zip_eq(parent2.chars())
    {
        if c != p1 && c != p2 {
            return false;
        }
    }
    true
}

fn find_similarity(child: &str, parent1: &str, parent2: &str) -> i32 {
    let mut similarity = 1;
    for parent in &[parent1, parent2] {
        let mut sum = 0;
        for (s, o) in child.chars().zip_eq(parent.chars()) {
            sum += if s == o { 1 } else { 0 };
        }
        similarity *= sum;
    }
    similarity
}

pub fn solve(data: String) {
    println!("Text input: {}", data);
    let mut lines = data.lines();
    let one = lines.next().unwrap().split(":").nth(1).unwrap();
    let two = lines.next().unwrap().split(":").nth(1).unwrap();
    let three = lines.next().unwrap().split(":").nth(1).unwrap();
    if find_child(one, two, three) {
        println!("similarity: {}", find_similarity(one, two, three))
    } else if find_child(two, one, three) {
        println!("similarity: {}", find_similarity(two, one, three))
    } else if find_child(three, one, two) {
        println!("similarity: {}", find_similarity(three, one, two))
    } else {
        println!("child not found")
    }
}

pub fn solve2(data: String) {
    println!("Text input: {}", data);
    let scales = data
        .lines()
        .map(|s| Scale::from_str(s).unwrap())
        .collect::<Vec<Scale>>();
    let s = find_children(&scales);
    println!("total similarity: {}", s);
}

pub fn solve3(data: String) {
    println!("Text input: {}", data);
}
