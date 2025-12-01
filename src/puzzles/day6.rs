#[allow(dead_code)]
fn split_groups<T>(v: T) -> Vec<Vec<<T as IntoIterator>::Item>>
where
    T: IntoIterator,
    <T as IntoIterator>::Item: Eq,
    <T as IntoIterator>::Item: Copy,
{
    let mut out = Vec::new();
    let mut cur = vec![];
    let mut cur_char = None;

    for n in v.into_iter() {
        if n != cur_char.unwrap_or(n) {
            out.push(cur.clone());
            cur.clear();
        }
        cur.push(n);
        cur_char = Some(n.to_owned());
    }

    out.push(cur.clone());
    out
}

fn find_pairs(s: &str, mentor: char) -> usize {
    let mut iter = s.chars();
    let mut total_novices = 0;
    while iter.any(|c| c == mentor) {
        let new_iter = iter.clone();
        let novices = new_iter
            .filter(|&c| c == mentor.to_ascii_lowercase())
            .count();
        total_novices += novices;
    }
    total_novices
}

pub fn solve(data: String) {
    println!("Text input: {}", data);
    dbg!(find_pairs(&data, 'A'));
}

pub fn solve2(data: String) {
    println!("Text input: {}", data);
    dbg!(['A', 'B', 'C']
        .map(|x| find_pairs(&data, x))
        .iter()
        .sum::<usize>());
}

pub fn solve3(data: String) {
    println!("Text input: {}", data);
}
