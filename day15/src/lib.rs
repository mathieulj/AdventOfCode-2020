use displaydoc::Display;
use itertools::Itertools as _;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error, Display)]
pub enum Errors {
    /// Parse error {0}
    ParseError(#[from] std::num::ParseIntError),
}

pub fn game(mut starting_numbers: Vec<(u64, usize)>, nth: usize) -> u64 {
    let (last, last_index) = starting_numbers.pop().unwrap_or((0, 0));
    let mut spoken: HashMap<u64, usize> = starting_numbers.into_iter().collect();

    (last_index..nth - 1).fold(last, |last, index| {
        spoken
            .insert(last, index)
            .map(|last_spoken| (index - last_spoken) as u64)
            .unwrap_or(0)
    })
}

pub fn challenge1(input: &str) -> Result<u64, Errors> {
    let list: Vec<(u64, usize)> = input
        .split(',')
        .enumerate()
        .map(|(index, num)| num.parse::<u64>().map(|num| (num, index)))
        .try_collect()?;

    Ok(game(list, 2020))
}

pub fn challenge2(input: &str) -> Result<u64, Errors> {
    let list: Vec<(u64, usize)> = input
        .split(',')
        .enumerate()
        .map(|(index, num)| num.parse::<u64>().map(|num| (num, index)))
        .try_collect()?;

    Ok(game(list, 30000000))
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"0,3,6"#;

    #[test]
    fn test_challenge1() -> Result<(), super::Errors> {
        assert_eq!(super::challenge1(INPUT)?, 436);
        Ok(())
    }

    #[test]
    fn test_challenge2() -> Result<(), super::Errors> {
        assert_eq!(super::challenge2("0,3,6")?, 175594);
        assert_eq!(super::challenge2("1,3,2")?, 2578);
        assert_eq!(super::challenge2("2,1,3")?, 3544142);
        assert_eq!(super::challenge2("1,2,3")?, 261214);
        assert_eq!(super::challenge2("2,3,1")?, 6895259);
        assert_eq!(super::challenge2("3,2,1")?, 18);
        assert_eq!(super::challenge2("3,1,2")?, 362);
        Ok(())
    }
}
