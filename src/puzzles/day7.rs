use std::{collections::HashSet, str::FromStr};

use anyhow::Error;

#[derive(Debug)]
struct Teleporter {
    start: usize,
    beams: Vec<HashSet<usize>>,
    splitters: Vec<HashSet<usize>>,
    split_counts: usize,
    timelines: usize,
}

impl FromStr for Teleporter {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitters = Vec::new();
        let mut start = 0;
        for line in s.lines() {
            splitters.push(line.match_indices('^').map(|(idx, _)| idx).collect());
            if let Some(start_idx) = line.find('S') {
                start = start_idx;
            }
        }
        Ok(Self {
            start,
            beams: Vec::new(),
            splitters,
            split_counts: 0,
            timelines: 0,
        })
    }
}

impl Teleporter {
    fn fire_beam(&mut self) {
        let mut first = HashSet::new();
        first.insert(self.start);
        self.beams.push(first);
        let mut split_iter = self.splitters.iter();
        split_iter.next();

        for splits in split_iter {
            let last_beams = self.beams.last().expect("guaranteed not empty");
            let mut new_beams = HashSet::new();
            for b in last_beams {
                if splits.contains(b) {
                    new_beams.insert(b - 1);
                    new_beams.insert(b + 1);
                    self.split_counts += 1;
                    self.timelines += 2;
                } else {
                    new_beams.insert(*b);
                }
            }
            let _new_beam_count = new_beams.iter().filter(|b| !last_beams.contains(b)).count();
            self.beams.push(new_beams);
        }
    }
}

#[allow(clippy::unnecessary_wraps)]
pub fn solve(data: &str) -> Result<(String, String), Error> {
    let data = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";
    let mut teleporter = Teleporter::from_str(data)?;
    teleporter.fire_beam();
    Ok((
        teleporter.split_counts.to_string(),
        teleporter.timelines.to_string(),
    ))
}
