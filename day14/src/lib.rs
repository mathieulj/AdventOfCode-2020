#![allow(unstable_name_collisions)]
use std::{collections::HashMap, str::FromStr};

use displaydoc::Display;
use regex::Regex;
use thiserror::Error;

#[derive(Debug, Error, Display)]
pub enum Errors {
    /// Parse error {0}
    ParseError(#[from] std::num::ParseIntError),
    /// Could not parse {0}
    InvalidLine(String),
    /// Bad regex {0}
    BadRegex(#[from] regex::Error),
}

enum Instruction {
    Mask { clear: u64, float: u64, set: u64 },
    Assign { address: u64, value: u64 },
}

impl FromStr for Instruction {
    type Err = Errors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mask = Regex::new(r#"^mask = (?P<mask>[X10]+)$"#)?;
        let assign = Regex::new(r#"^mem\[(?P<address>\d+)\] = (?P<value>\d+)$"#)?;
        if let Some(captures) = mask.captures(s) {
            let mut set = 0;
            let mut clear = 0;
            let mut float = 0;

            for c in captures["mask"].chars() {
                set <<= 1;
                clear <<= 1;
                float <<= 1;
                match c {
                    '1' => set |= 1,
                    '0' => clear |= 1,
                    'X' => float |= 1,
                    _ => {}
                }
            }

            Ok(Self::Mask { clear, float, set })
        } else if let Some(captures) = assign.captures(s) {
            let address = captures["address"].parse()?;
            let value = captures["value"].parse()?;
            Ok(Self::Assign { address, value })
        } else {
            Err(Errors::InvalidLine(s.to_string()))
        }
    }
}

pub fn challenge1(input: &str) -> Result<u64, Errors> {
    let mut mem = HashMap::new();
    let mut set_mask = 0;
    let mut clear_mask = 0;

    for line in input.lines() {
        let instruction: Instruction = line.parse()?;

        match instruction {
            Instruction::Mask { set, clear, .. } => {
                set_mask = set;
                clear_mask = clear;
            }
            Instruction::Assign { address, value } => {
                mem.insert(address, set_mask | value & !clear_mask);
            }
        }
    }
    Ok(mem.values().sum())
}

pub fn challenge2(input: &str) -> Result<u64, Errors> {
    let mut mem = HashMap::new();
    let mut float_mask = 0;
    let mut set_mask = 0;

    for line in input.lines() {
        let instruction: Instruction = line.parse()?;

        match instruction {
            Instruction::Mask { set, float, .. } => {
                set_mask = set;
                float_mask = float;
            }
            Instruction::Assign { address, value } => {
                mem.extend(
                    float_mask_options(address | set_mask, float_mask)
                        .map(|address| (address, value)),
                );
            }
        }
    }
    Ok(mem.values().sum())
}

pub fn float_mask_options(base: u64, mask: u64) -> impl Iterator<Item = u64> {
    // Build a mapping to spread permutation over all masked bits without having to go through all possible integers
    let bit_mappings: Vec<_> = (0..36)
        .map(|b| 1 << b)
        .filter(|base_mask| mask & base_mask != 0)
        .enumerate()
        .map(|(index, base_mask)| (1 << index, base_mask))
        .collect();

    (0..1 << bit_mappings.len()).map(move |permutation| {
        bit_mappings
            .iter()
            .fold(base, |value, (permutation_mask, base_mask)| {
                if permutation_mask & permutation == 0 {
                    value & !base_mask
                } else {
                    value | base_mask
                }
            })
    })
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_challenge1() -> Result<(), super::Errors> {
        const INPUT: &str = r#"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"#;
        assert_eq!(super::challenge1(INPUT)?, 165);
        Ok(())
    }

    #[test]
    fn test_challenge2() -> Result<(), super::Errors> {
        const INPUT: &str = r#"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"#;
        assert_eq!(super::challenge2(INPUT)?, 208);
        Ok(())
    }
}
