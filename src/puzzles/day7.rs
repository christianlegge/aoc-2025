use std::{char, fmt::format, str::FromStr};

use hashbrown::{HashMap, HashSet};
use itertools::{all, any};

#[derive(Debug)]
struct Grammar {
    rules: Vec<Rule>,
    memo_map: HashMap<String, u32>,
    memo_set: HashSet<String>,
}

#[derive(Debug, Clone)]
struct Rule {
    start: char,
    follows: String,
}

impl Grammar {
    pub fn valid(&self, s: &str) -> bool {
        all(0..s.len() - 1, |i| {
            any(&self.rules, |rule| rule.valid(&s[i..]))
        })
    }

    pub fn count_strings(&mut self, prefix: String) -> u32 {
        if let Some(n) = self.memo_map.get(&prefix) {
            return *n;
        }
        if !self.valid(&prefix) {
            self.memo_map.insert(prefix, 0);
            return 0;
        }
        if prefix.len() == 11 {
            self.memo_map.insert(prefix.clone(), 1);
            self.memo_set.insert(prefix);
            return 1;
        }
        let last_char = prefix.chars().last().unwrap();
        let count_self = if prefix.len() < 7 {
            0
        } else {
            self.memo_set.insert(prefix.clone());
            1
        };
        let char_rule = self.get_char_rule(last_char).cloned();
        match char_rule {
            None => {
                self.memo_map.insert(prefix, count_self);
                count_self
            }
            Some(rule) => {
                let count = count_self
                    + rule
                        .follows
                        .chars()
                        .map(|c| self.count_strings(format(format_args!("{prefix}{c}"))))
                        .sum::<u32>();
                self.memo_map.insert(prefix, count);
                count
            }
        }
    }

    fn get_char_rule(&self, c: char) -> Option<&Rule> {
        self.rules.iter().find(|rule| rule.start == c)
    }
}

impl Rule {
    pub fn valid(&self, s: &str) -> bool {
        // println!("checking validity: {:#?} {}", self, s);
        let mut chars = s.chars();
        chars.next().unwrap_or('\0') == self.start
            && self.follows.contains(chars.next().unwrap_or('\0'))
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" > ");
        Ok(Rule {
            start: s.chars().next().unwrap(),
            follows: parts.nth(1).unwrap().replace(",", ""),
        })
    }
}

pub fn solve(data: String) {
    println!("Text input: {}", data);
    let mut lines = data.lines();
    let mut names = lines.next().unwrap().split(",");
    lines.next();
    let g = Grammar {
        rules: lines.map(|line| Rule::from_str(line).unwrap()).collect(),
        memo_map: HashMap::new(),
        memo_set: HashSet::new(),
    };

    let name = names.find(|&name| g.valid(name)).unwrap();

    println!("name found: {}", name);

    // let rs = vec![
    //     Rule {
    //         start: 'a',
    //         follows: String::from("bcd"),
    //     },
    //     Rule {
    //         start: 'b',
    //         follows: String::from("cde"),
    //     },
    // ];
    // let g = Grammar { rules: rs };
    // assert!(g.valid(String::from("abc")));
    // assert!(!g.valid(String::from("aba")));
}

pub fn solve2(data: String) {
    println!("Text input: {}", data);
    let mut lines = data.lines();
    let names = lines.next().unwrap().split(",");
    lines.next();
    let g = Grammar {
        rules: lines.map(|line| Rule::from_str(line).unwrap()).collect(),
        memo_map: HashMap::new(),
        memo_set: HashSet::new(),
    };

    let index_sum = names
        .enumerate()
        .filter(|(_, name)| g.valid(name))
        .map(|(i, _)| i + 1)
        .sum::<usize>();

    dbg!(index_sum);
}

pub fn solve3(data: String) {
    println!("Text input: {}", data);

    let mut lines = data.lines();
    let names = lines.next().unwrap().split(",");
    lines.next();
    let mut g = Grammar {
        rules: lines.map(|line| Rule::from_str(line).unwrap()).collect(),
        memo_map: HashMap::new(),
        memo_set: HashSet::new(),
    };

    for ele in names {
        println!("strings for {ele}: {}", g.count_strings(String::from(ele)));
    }

    println!("{}", g.memo_set.len());
}
