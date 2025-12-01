use std::str::FromStr;

struct Dial {
    position: i32,
    max: i32,
    zeroes: i32,
}

impl Dial {
    fn make_move(&mut self, m: &Move) {
        match *m {
            Move::Left(i) => {
                for _ in 0..i {
                    self.position -= 1;
                    if self.position == 0 {
                        self.zeroes += 1;
                    }
                    if self.position == -1 {
                        self.position = self.max - 1;
                    }
                }
            }
            Move::Right(i) => {
                for _ in 0..i {
                    self.position += 1;
                    if self.position == self.max {
                        self.position = 0;
                    }
                    if self.position == 0 {
                        self.zeroes += 1;
                    }
                }
            }
        };
    }

    fn count_zeroes(&self, m: &Move) -> i32 {
        let (dist_to_zero, move_dist) = match *m {
            Move::Left(i) => (self.position, i),
            Move::Right(i) => (self.max - self.position, i),
        };
        let extra_move = if self.position == 0 { 0 } else { 1 };
        if dist_to_zero > move_dist {
            0
        } else {
            let rem = move_dist - dist_to_zero;
            extra_move + rem / self.max
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
            Some('L') => Ok(Move::Left(s[1..].parse()?)),
            Some('R') => Ok(Move::Right(s[1..].parse()?)),
            _ => Err(anyhow::anyhow!("error parsing string {s}")),
        }
    }
}

impl ToString for Move {
    fn to_string(&self) -> String {
        match self {
            Move::Left(i) => format!("L{i}"),
            Move::Right(i) => format!("R{i}"),
        }
    }
}

pub fn solve(data: String) {
    println!("Text input: {}", data);
    let mut dial = Dial {
        position: 50,
        max: 100,
        zeroes: 0,
    };
    let mut zeroes = 0;
    let mut more_zeroes = 0;
    for m in data.lines().map(|line| Move::from_str(line).unwrap()) {
        let passes = dial.count_zeroes(&m);
        dial.make_move(&m);
        if dial.position == 0 {
            zeroes += 1;
        }
        more_zeroes += passes;
        println!(
            "rotated {} to point at {}, passed {} times",
            m.to_string(),
            dial.position,
            passes
        );
    }

    dbg!(zeroes, more_zeroes, dial.zeroes);
}

pub fn solve2(data: String) {
    println!("Text input: {}", data);
}

pub fn solve3(data: String) {
    println!("Text input: {}", data);
}
