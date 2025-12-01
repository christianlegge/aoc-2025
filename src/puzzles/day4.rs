use aoc_2025::util::line_split_numbers;

pub fn solve(data: String) {
    println!("Text input: {}", data);
    let inputs = line_split_numbers::<i32>(data);

    println!(
        "final result: {}",
        2025 * inputs[0] / inputs[inputs.len() - 1]
    );
}

pub fn solve2(data: String) {
    println!("Text input: {}", data);
    let inputs = line_split_numbers::<i32>(data);

    println!(
        "final result: {}",
        (10000000000000f64 * inputs[inputs.len() - 1] as f64 / inputs[0] as f64).ceil()
    );
}

#[derive(Debug)]
enum Gear {
    Single(u64),
    Coupled(u64, u64),
}

pub fn solve3(data: String) {
    let lines = data.split_whitespace();
    let mut gears: Vec<Gear> = Vec::new();
    for line in lines {
        let parts = line.split("|").collect::<Vec<&str>>();
        gears.push(match parts.len() {
            1 => Gear::Single(parts[0].parse::<u64>().unwrap()),
            2 => Gear::Coupled(
                parts[0].parse::<u64>().unwrap(),
                parts[1].parse::<u64>().unwrap(),
            ),
            _ => panic!("something happened {} {:#?}", line, parts),
        })
    }

    if let Gear::Single(first_gear) = gears[0] {
        println!(
            "final: {:#?}",
            gears
                .iter()
                .fold((first_gear, 100f64), |a: (u64, f64), v| {
                    println!("{} turns; gear: {:#?}", a.1, v);
                    match v {
                        Gear::Single(x) => (*x, a.1 * a.0 as f64 / *x as f64),
                        Gear::Coupled(x, y) => (*y, a.1 * a.0 as f64 / *x as f64),
                    }
                })
                .1
                .floor()
        )
    }
}
