use std::rc::Rc;

use anyhow::Error;
use num::{abs, integer::sqrt};

#[derive(Debug)]
struct JunctionBox {
    x: u64,
    y: u64,
    z: u64,
    connections: Vec<Rc<JunctionBox>>,
}

impl JunctionBox {
    fn dist(&self, other: &JunctionBox) -> u64 {
        sqrt(
            self.x.abs_diff(other.x).pow(2)
                + self.y.abs_diff(other.y).pow(2)
                + self.z.abs_diff(other.z).pow(2),
        )
    }

    fn connect(&mut self, other: &mut JunctionBox) {
        // other.connections.push(Rc::clone(*self));
        // self.connections.push(Rc::new(*other));
    }
}

fn connect(first: Rc<JunctionBox>, second: Rc<JunctionBox>) {
    // first.connections.push(Rc::clone(&second));
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
    Ok(("Unimplemented".to_string(), "Unimplemented".to_string()))
}
