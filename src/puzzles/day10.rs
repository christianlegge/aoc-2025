use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug)]
struct Board {
    squares: Vec<Square>,
    rows: usize,
    cols: usize,
}

#[derive(Debug, PartialEq)]
enum Square {
    Dragon,
    Sheep,
    Empty,
}

impl TryFrom<char> for Square {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'D' => Ok(Square::Dragon),
            'S' => Ok(Square::Sheep),
            '.' => Ok(Square::Empty),
            c => Err(format!("Unknown board character {c}")),
        }
    }
}

impl FromStr for Board {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows = 0;
        let mut cols = 0;
        let mut squares = Vec::new();

        for line in s.lines() {
            rows += 1;
            cols = line.len();
            for c in line.chars() {
                squares.push(Square::try_from(c)?);
            }
        }

        match cols {
            0 => Err(format!("error counting cols: {s}")),
            _ => Ok(Board {
                rows,
                cols,
                squares,
            }),
        }
    }
}

impl Board {
    pub fn get_square(&self, row: usize, col: usize) -> Option<&Square> {
        let idx = row * self.cols + col;
        self.squares.get(idx)
    }

    pub fn get_coords(&self, idx: usize) -> (usize, usize) {
        (idx / self.cols, idx % self.cols)
    }

    pub fn find_dragon(&self) -> Option<(usize, usize)> {
        match self.squares.iter().find_position(|&s| s == &Square::Dragon) {
            Some(i) => Some(self.get_coords(i.0)),
            None => None,
        }
    }
}

fn get_knight_moves(row: usize, col: usize, clamp: usize) -> Vec<(usize, usize)> {
    let row = row as i32;
    let col = col as i32;
    let clamp = clamp as i32;
    let mut moves = Vec::new();
    for (i, j) in &[
        (2, 1),
        (-2, 1),
        (2, -1),
        (-2, -1),
        (1, 2),
        (1, -2),
        (-1, 2),
        (-1, -2),
    ] {
        if row + i < clamp && col + j < clamp && row + i >= 0 && col + j >= 0 {
            moves.push(((row + i) as usize, (col + j) as usize));
        }
    }
    moves
}

pub fn solve(data: String) {
    println!("Text input: {}", data);
    let board = match Board::from_str(&data) {
        Ok(b) => b,
        Err(s) => panic!("{s}"),
    };
    let dragon = match board.find_dragon() {
        Some(d) => d,
        None => panic!("no dragon found!"),
    };

    let mut moves = vec![dragon];
    let mut total_moves = moves.clone();
    for _ in 0..4 {
        moves = moves
            .iter()
            .flat_map(|m| get_knight_moves(m.0, m.1, board.rows))
            .collect::<Vec<(usize, usize)>>();
        total_moves.append(&mut moves.clone());
    }

    let dead_sheep = total_moves
        .iter()
        .filter(|(i, j)| board.get_square(*i, *j) == Some(&Square::Sheep))
        .collect::<Vec<_>>()
        .len();

    dbg!(total_moves, dead_sheep);
}

pub fn solve2(data: String) {
    println!("Text input: {}", data);
}

pub fn solve3(data: String) {
    println!("Text input: {}", data);
}
