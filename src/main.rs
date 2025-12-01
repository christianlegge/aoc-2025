mod puzzles;

use seq_macro::seq;

#[allow(clippy::cognitive_complexity)]
fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 1 {
        println!("No arguments provided");
    } else {
        let arg = args[1].as_str();
        let data = std::fs::read_to_string(format!("data/aoc_2025_{arg}"));
        seq!(N in 1..=12 {
            match arg {
            #(
            stringify!(day~N) => {
                match data {
                    Ok(data) => {
                        match crate::puzzles::day~N::solve(&data) {
                        Ok((s1, s2)) => {
                            println!("Part 1 solution: {s1}");
                            println!("Part 2 solution: {s2}");
                        }
                        Err(e) => println!("Error solving puzzle: {e}")
                    }
                    }
                    Err(err) => println!("Error reading data 1: {err}"),
                }
            }
            )*
                _ => panic!("Invalid argument {arg}")
            }
        })
    }
}
