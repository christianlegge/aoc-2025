use std::str::FromStr;

use anyhow::Error;
use itertools::Itertools;

#[derive(Debug)]
struct BoxNetwork {
    boxes: Vec<JunctionBox>,
    connections: Vec<(usize, usize)>,
}

#[derive(Debug)]
struct JunctionBox {
    x: u64,
    y: u64,
    z: u64,
}

impl BoxNetwork {
    fn find_next_shortest(&self, at_least: f64) -> (f64, (usize, usize)) {
        let mut min = f64::INFINITY;
        let mut ret = (0, 0);
        for (ia, a) in self.boxes.iter().enumerate() {
            for (ib, b) in self.boxes.iter().enumerate() {
                if ia == ib {
                    continue;
                }
                let dist = a.dist(b);
                if dist < min && dist > at_least {
                    ret = (ia, ib);
                    min = dist;
                }
            }
        }
        (min, ret)
    }

    fn make_shortest_connections(&mut self, num: usize) {
        let mut longest_made = 0.0;
        for _ in 0..num {
            let (dist, (a, b)) = self.find_next_shortest(longest_made);
            longest_made = dist;
            self.connections.push((a, b));
        }
    }

    fn print_connections(&self) {
        println!("connections:");
        for &(a, b) in &self.connections {
            println!("{:?}, {:?}", self.boxes[a], self.boxes[b]);
        }
    }
}

impl JunctionBox {
    fn dist(&self, other: &Self) -> f64 {
        #[allow(clippy::cast_precision_loss)]
        f64::sqrt(
            (self.x.abs_diff(other.x).pow(2)
                + self.y.abs_diff(other.y).pow(2)
                + self.z.abs_diff(other.z).pow(2)) as f64,
        )
    }
}

impl FromStr for JunctionBox {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        Ok(Self {
            x: parts
                .next()
                .ok_or_else(|| anyhow::anyhow!("missing x coord"))?
                .parse()?,
            y: parts
                .next()
                .ok_or_else(|| anyhow::anyhow!("missing y coord"))?
                .parse()?,
            z: parts
                .next()
                .ok_or_else(|| anyhow::anyhow!("missing z coord"))?
                .parse()?,
        })
    }
}

#[allow(clippy::unnecessary_wraps)]
pub fn solve(data: &str) -> Result<(String, String), Error> {
    let data = r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    println!("Text input: {data}");
    let points: Vec<_> = data.lines().map(JunctionBox::from_str).try_collect()?;

    let mut network = BoxNetwork {
        boxes: points,
        connections: Vec::new(),
    };

    network.make_shortest_connections(10);
    network.print_connections();

    dbg!(&network);

    Ok(("Unimplemented".to_string(), "Unimplemented".to_string()))
}
