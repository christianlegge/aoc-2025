use std::{fmt::Display, str::FromStr};

use anyhow::Error;

#[derive(Debug)]
struct Grid {
    squares: Vec<Square>,
    rows: i32,
    cols: i32,
    paper_removed: i32,
}

#[derive(Debug)]
struct GridIterator {
    cur: (i32, i32),
    rows: i32,
    cols: i32,
}

impl Iterator for GridIterator {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.cur {
            (r, _) if r == self.rows => None,
            (r, c) => Some((r, c)),
        };
        self.cur = match self.cur {
            (r, c) if c == self.cols - 1 => (r + 1, 0),
            (r, c) => (r, c + 1),
        };

        ret
    }
}

#[derive(Debug, PartialEq)]
enum Square {
    Paper,
    Empty,
}

impl IntoIterator for Grid {
    type Item = (i32, i32);

    type IntoIter = GridIterator;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            cur: (0, 0),
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl Grid {
    pub const fn iter(&self) -> GridIterator {
        GridIterator {
            cur: (0, 0),
            rows: self.rows,
            cols: self.cols,
        }
    }

    pub fn get_square(&self, row: i32, col: i32) -> Option<&Square> {
        if row < 0 || row >= self.rows || col < 0 || col >= self.cols {
            None
        } else {
            let idx = usize::try_from(row * self.cols + col);
            idx.map_or(None, |idx| self.squares.get(idx))
        }
    }

    pub fn get_adjacent_squares(&self, row: i32, col: i32) -> Vec<((i32, i32), &Square)> {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .filter_map(|(i, j)| self.get_square(row + i, col + j).map(|s| ((*i, *j), s)))
        .collect()
    }

    pub fn remove_paper(&mut self, paper: impl IntoIterator<Item = (i32, i32)>) {
        for (i, j) in paper {
            self.squares[usize::try_from(i * self.cols + j)
                .expect("error converting paper square to index")] = Square::Empty;
            self.paper_removed += 1;
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(
                    f,
                    "{}",
                    self.get_square(i, j).expect("Checked invalid square")
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for Grid {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows = 0;
        let mut cols = 0;
        let mut squares = Vec::new();

        for line in s.lines() {
            rows += 1;
            cols = i32::try_from(line.len())?;
            for c in line.chars() {
                let square = Square::try_from(c)?;
                squares.push(square);
            }
        }

        match cols {
            0 => Err(anyhow::anyhow!("error counting cols: {s}")),
            _ => Ok(Self {
                squares,
                rows,
                cols,
                paper_removed: 0,
            }),
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Paper => '@',
                Self::Empty => '.',
            }
        )
    }
}

impl TryFrom<char> for Square {
    type Error = Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '@' => Ok(Self::Paper),
            '.' => Ok(Self::Empty),
            c => Err(anyhow::anyhow!("Unknown board character {c}")),
        }
    }
}
#[allow(clippy::unnecessary_wraps)]
pub fn solve(data: &str) -> Result<(String, String), Error> {
    let mut grid = Grid::from_str(data.trim())?;
    let mut count = 0;
    dbg!(grid.get_adjacent_squares(1, 0));
    for (i, j) in grid.iter() {
        // println!("{i}, {j}");
        if grid.get_square(i, j) == Some(&Square::Paper) {
            let adj = grid.get_adjacent_squares(i, j);
            if adj.iter().filter(|&&s| s.1 == &Square::Paper).count() < 4 {
                count += 1;
            }
        }
    }
    loop {
        let mut removable = Vec::new();
        for (i, j) in grid.iter() {
            if grid.get_square(i, j) == Some(&Square::Paper) {
                let adj = grid.get_adjacent_squares(i, j);
                if adj.iter().filter(|&&s| s.1 == &Square::Paper).count() < 4 {
                    removable.push((i, j));
                }
            }
        }
        if removable.is_empty() {
            break;
        }
        grid.remove_paper(removable);
    }
    Ok((count.to_string(), grid.paper_removed.to_string()))
}
