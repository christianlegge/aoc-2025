use aoc_2025::util::comma_split_numbers;
use itertools::Itertools;

pub fn solve(data: String) {
    println!("Text input: {}", data);
    let nums = comma_split_numbers::<i32>(data);
    let mut total = 0;
    for (&x, &y) in nums.iter().tuple_windows() {
        println!("{} {}", x, y);
        total += if (x - y).abs() == 16 { 1 } else { 0 };
    }
    // let passes = nums
    //     .iter()
    //     .tuple_windows()
    //     .filter(|(&x, &y)| (x - y).abs() == 16)
    //     .try_len()
    //     .unwrap();
    dbg!(total);
}

pub fn solve2(data: String) {
    println!("Text input: {}", data);
    // let data = "1,5,2,6,8,4,1,7,3,5,7,8,2";
    let pegs = 256;
    let nums = comma_split_numbers::<i32>(data.to_string());
    let mut crosses = 0;
    for i in 0..nums.len() {
        if i < 2 {
            println!("step {} invalid, skipping", i + 1);
            continue;
        }
        let cur = &nums[i];
        let last = &nums[i - 1];
        let split = (last - cur).rem_euclid(pegs);
        let mut local_crosses = 0;
        for (&x, &y) in nums[0..i - 1].iter().tuple_windows() {
            if x == *cur || y == *cur || x == *last || y == *last {
                continue;
            }
            if ((x - cur).rem_euclid(pegs) > split) != ((y - cur).rem_euclid(pegs) > split) {
                println!("counting string {}-{}", x, y);
                local_crosses += 1;
            }
        }
        crosses += local_crosses;
        dbg!(local_crosses);
    }
    dbg!(crosses);
}

pub fn solve3(data: String) {
    println!("Text input: {}", data);
}
