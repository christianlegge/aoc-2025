use std::str::FromStr;

use anyhow::Error;
use itertools::Itertools;

/// # Errors
///
/// Returns an error if any of the strings fail to parse.
pub fn parse_csv<T: FromStr<Err = anyhow::Error>>(s: &str) -> Result<Vec<T>, Error> {
    s.split(',')
        .map(|x| T::from_str(x))
        .try_collect::<T, Vec<T>, Error>()
}

/// # Errors
///
/// Returns an error if any of the strings fail to parse.
pub fn parse_lines<T: FromStr<Err = anyhow::Error>>(s: &str) -> Result<Vec<T>, Error> {
    s.lines()
        .map(|x| T::from_str(x))
        .try_collect::<T, Vec<T>, Error>()
}
