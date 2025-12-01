mod puzzles;

use crate::puzzles::*;
use seq_macro::seq;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    match args.len() {
        1 => {
            println!("No arguments provided");
        }
        _ => {
            let arg = args[1].as_str();
            let data = std::fs::read_to_string(format!("data/aoc_2025_{}", arg));
            // let data2 =
            //     std::fs::read_to_string(format!("data/everybody_codes_e2025_{}_p2.txt", arg));
            // let data3 =
            //     std::fs::read_to_string(format!("data/everybody_codes_e2025_{}_p3.txt", arg));
            seq!(N in 1..=20 {
                match arg {
                #(
                stringify!(day~N) => {
                    match data {
                        Ok(data) => {
                            day~N::solve(data.clone());
                        }
                        Err(err) => println!("Error reading data 1: {}", err),
                    };
                    // println!("============== PART 2 ==============");
                    // match data {
                    //     Ok(data) => day~N::solve2(data),
                    //     Err(err) => println!("Error reading data 2: {}", err),
                    // };
                    // println!("============== PART 3 ==============");
                    // match data3 {
                    //     Ok(data) => day~N::solve3(data),
                    //     Err(err) => println!("Error reading data 3: {}", err),
                    // };
                }
                )*
                    _ => panic!("Invalid argument {}", arg),
                }

            })
        }
    }
}
