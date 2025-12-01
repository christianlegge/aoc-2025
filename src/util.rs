pub fn comma_split_numbers<T: std::str::FromStr>(s: String) -> Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    s.split(",")
        .map(|x| x.parse::<T>().unwrap())
        .collect::<Vec<T>>()
}

pub fn line_split_numbers<T: std::str::FromStr>(s: String) -> Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    s.split("\n")
        .map(|x| x.parse::<T>().unwrap())
        .collect::<Vec<T>>()
}
