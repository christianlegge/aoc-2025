use std::str::FromStr;

use itertools::Itertools;

/// # Errors
///
/// Returns an error if any of the strings fail to parse.
pub fn parse_csv<T: FromStr>(s: &str) -> Result<Vec<T>, <T as FromStr>::Err> {
    s.split(',')
        .map(|x| T::from_str(x))
        .try_collect::<T, Vec<T>, <T as FromStr>::Err>()
}

/// # Errors
///
/// Returns an error if any of the strings fail to parse.
pub fn parse_lines<T: FromStr>(s: &str) -> Result<Vec<T>, <T as FromStr>::Err> {
    s.lines()
        .map(|x| T::from_str(x))
        .try_collect::<T, Vec<T>, <T as FromStr>::Err>()
}
