use displaydoc::Display;
use itertools::Itertools as _;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    convert::TryFrom,
    str::FromStr,
};
use thiserror::Error;

#[derive(Debug, Error, Display)]
pub enum Errors {
    /// Parse error {0}
    ParseError(#[from] std::num::ParseIntError),
    /// Regex parse error {0}
    RegexError(#[from] regex::Error),
    /// No solution found
    NoSolution,
    /// Input not of the expected format
    BadInput,
    /// Line not of the expected format
    BadLine(String),
    /// Range format invalid
    BadRange,
}

#[derive(Debug, Clone)]
struct Range(u64, u64);
impl Range {
    pub fn contains(&self, value: u64) -> bool {
        self.0 <= value && value <= self.1
    }
}

impl FromStr for Range {
    type Err = Errors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split('-').collect_tuple().ok_or(Errors::BadRange)?;
        Ok(Self(a.parse()?, b.parse()?))
    }
}

#[derive(Debug, Clone)]
struct Constraint<'t> {
    pub name: &'t str,
    pub r1: Range,
    pub r2: Range,
}

impl Constraint<'_> {
    fn is_valid(&self, value: u64) -> bool {
        self.r1.contains(value) || self.r2.contains(value)
    }
}

impl<'t> TryFrom<&'t str> for Constraint<'t> {
    type Error = Errors;

    fn try_from(line: &'t str) -> Result<Self, Self::Error> {
        let c_re = regex::Regex::new(r#"^(.+): (\d+-\d+) or (\d+-\d+)$"#)?;
        let captures = c_re
            .captures(line)
            .ok_or(Errors::BadLine(line.to_string()))?;

        let name = captures.get(1).unwrap().as_str();
        let r1 = captures[2].parse()?;
        let r2 = captures[3].parse()?;

        Ok(Self { name, r1, r2 })
    }
}

fn parse(input: &str) -> Result<(Vec<Constraint>, Vec<u64>, Vec<Vec<u64>>), Errors> {
    let (constraints, my_ticket, nearby_tickets) = input
        .split("\n\n")
        .collect_tuple()
        .ok_or(Errors::BadInput)?;

    let constraints = constraints
        .lines()
        .map(Constraint::try_from)
        .try_collect()?;

    let mine: Vec<u64> = my_ticket
        .lines()
        .skip(1)
        .next()
        .ok_or(Errors::BadInput)?
        .split(',')
        .map(str::parse)
        .try_collect()?;

    let nearby: Vec<Vec<u64>> = nearby_tickets
        .lines()
        .skip(1)
        .map(|line| line.split(',').map(str::parse).try_collect())
        .try_collect()?;

    Ok((constraints, mine, nearby))
}

pub fn challenge1(input: &str) -> Result<u64, Errors> {
    let (constraints, _, nearby) = parse(input)?;

    let sum = nearby
        .iter()
        .flat_map(|c| c.iter())
        .filter(|&&n| {
            let valid = constraints.iter().any(|c| c.is_valid(n));

            !valid
        })
        .sum();

    Ok(sum)
}

pub fn challenge2(input: &str, key: &str) -> Result<u64, Errors> {
    let (constraints, mine, nearby) = parse(input)?;
    let mut possible_fields: Vec<Vec<Constraint>> = nearby
        .get(0)
        .ok_or(Errors::NoSolution)?
        .iter()
        .map(|_| constraints.clone())
        .collect();

    nearby
        .iter()
        .filter(|&n| n.iter().all(|&n| constraints.iter().any(|c| c.is_valid(n))))
        .flat_map(|valid_nearby| valid_nearby.iter().enumerate())
        .for_each(|(index, value)| {
            possible_fields[index].retain(|constraint| constraint.is_valid(*value));
        });

    let mut changes = true;
    while changes {
        changes = false;
        let confirmed_fields: Vec<&str> = possible_fields
            .iter()
            .filter(|f| f.len() == 1)
            .map(|f| f[0].name)
            .collect();

        for options in possible_fields.iter_mut() {
            let len = options.len();
            if len != 1 {
                options.retain(|c| !confirmed_fields.contains(&c.name));
            }
            changes |= options.len() != len;
        }
    }

    let mut product = 1;

    for (constraints, mine) in possible_fields.into_iter().zip(mine.into_iter()) {
        if constraints.len() != 1 {
            return Err(Errors::NoSolution);
        }

        if constraints[0].name.starts_with(key) {
            product *= mine;
        }
    }

    Ok(product)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_challenge1() -> Result<(), super::Errors> {
        const INPUT: &str = r#"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"#;
        assert_eq!(super::challenge1(INPUT)?, 71);
        Ok(())
    }

    #[test]
    fn test_challenge2() -> Result<(), super::Errors> {
        const INPUT: &str = r#"class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"#;
        assert_eq!(super::challenge2(INPUT, "s")?, 13);
        assert_eq!(super::challenge2(INPUT, "ro")?, 11);
        assert_eq!(super::challenge2(INPUT, "class")?, 12);
        Ok(())
    }
}
