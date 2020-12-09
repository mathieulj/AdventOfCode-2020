#![allow(unstable_name_collisions)]
use displaydoc::Display;
use itertools::Itertools as _;
use std::cmp::Ordering;
use std::ops::Not as _;
use thiserror::Error;
use utils::BoolExt as _;

#[derive(Debug, Error, Display)]
pub enum Errors {
    /// Parse error {0}
    ParseError(#[from] std::num::ParseIntError),
    /// Bad regex {0}
    BadRegex(#[from] regex::Error),
    /// No solution was found
    NoSolutionFound,
}

fn first_invalid(cipher_text: &Vec<u64>, preamble_len: usize) -> Result<u64, Errors> {
    (preamble_len..cipher_text.len())
        .find_map(|i| {
            cipher_text[i - preamble_len..i]
                .iter()
                .combinations(2)
                .any(|c| c[0] + c[1] == cipher_text[i])
                .not()
                .then(|| cipher_text[i])
        })
        .ok_or(Errors::NoSolutionFound)
}

pub fn challenge1(input: &str, preamble_len: usize) -> Result<u64, Errors> {
    let cipher_text: Vec<u64> = input.lines().map(str::parse).try_collect()?;
    first_invalid(&cipher_text, preamble_len)
}

pub fn challenge2(input: &str, preamble_len: usize) -> Result<u64, Errors> {
    let cipher_text: Vec<u64> = input.lines().map(str::parse).try_collect()?;
    let target = first_invalid(&cipher_text, preamble_len)?;

    let mut min_ptr = 0;
    let mut max_ptr = 2;

    while max_ptr <= cipher_text.len() {
        let range = &cipher_text[min_ptr..max_ptr];
        let sum = range.iter().sum::<u64>();

        match sum.cmp(&target) {
            Ordering::Equal => {
                return range
                    .iter()
                    .minmax()
                    .into_option()
                    .map(|(min, max)| min + max)
                    .ok_or(Errors::NoSolutionFound);
            }
            Ordering::Greater if max_ptr > min_ptr + 2 => min_ptr += 1,
            _ => max_ptr += 1,
        }
    }

    Err(Errors::NoSolutionFound)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"#;

    #[test]
    fn test_challenge1() -> Result<(), super::Errors> {
        assert_eq!(super::challenge1(INPUT, 5)?, 127);
        Ok(())
    }

    #[test]
    fn test_challenge2() -> Result<(), super::Errors> {
        assert_eq!(super::challenge2(INPUT, 5)?, 62);
        Ok(())
    }
}
