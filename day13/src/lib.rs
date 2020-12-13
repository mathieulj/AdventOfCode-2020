#![allow(unstable_name_collisions)]
use displaydoc::Display;
use itertools::Itertools as _;
use thiserror::Error;
use utils::BoolExt as _;

#[derive(Debug, Error, Display)]
pub enum Errors {
    /// Parse error {0}
    ParseError(#[from] std::num::ParseIntError),
    /// No solution was found
    NoSolutionFound,
    /// Invalid input
    InvalidInput,
}

pub fn challenge1(input: &str) -> Result<u64, Errors> {
    let lines = input.lines().collect::<Vec<&str>>();
    let departure_time: u64 = lines.get(0).ok_or(Errors::InvalidInput)?.parse()?;
    let bus_ids: Vec<u64> = lines
        .get(1)
        .ok_or(Errors::InvalidInput)?
        .split(',')
        .filter(|&s| s != "x")
        .map(str::parse)
        .try_collect()?;

    let (next_id, next_departure) = bus_ids
        .into_iter()
        .map(|id| {
            let instance = departure_time / id;
            (id, id * (instance + 1))
        })
        .min_by_key(|&(_id, next_departure)| next_departure)
        .ok_or(Errors::NoSolutionFound)?;

    Ok((next_departure - departure_time) * next_id)
}

pub fn challenge2(input: &str) -> Result<i64, Errors> {
    // Vec<(remainder, modulus)>
    let constraints: Vec<(i64, i64)> = input
        .lines()
        .nth(1)
        .ok_or(Errors::InvalidInput)?
        .split(',')
        .enumerate()
        .filter_map(|(index, s)| {
            // We want to find a number where items have a remainder of (modulus - index)
            // - `N % modulus[0] == 0`
            // - `(N + 1) % modulus[1] == 0` => `N % modulus[1] == modulus[1] - 1`
            // - ...
            (s != "x").then(|| {
                s.parse()
                    .map(|modulus: i64| (modulus - index as i64, modulus))
            })
        })
        .try_collect()?;

    // The product of the modulus is the repeat rate of the alignment
    let product: i64 = constraints.iter().map(|&(_, m)| m).product();
    let sum: i64 = constraints
        .iter()
        .map(|&(remainder, modulus)| {
            // Find a factor that will hide this contribution from the other modulus
            let other_mods = product / modulus;

            // Find a factor that will produce a remainder of 1 with this modulus `other_mods * inv % modulus == 1`
            let inv = (1..)
                .filter(|i| i * other_mods % modulus == 1)
                .next()
                .unwrap();

            // `other_mods * inv % modulus == 1` so multiplying by our remainder carries it into the result
            remainder * other_mods * inv
        })
        .sum();

    Ok(sum % product)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"939
7,13,x,x,59,x,31,19"#;

    #[test]
    fn test_challenge1() -> Result<(), super::Errors> {
        assert_eq!(super::challenge1(INPUT)?, 295);
        Ok(())
    }

    #[test]
    fn test_challenge2() -> Result<(), super::Errors> {
        assert_eq!(super::challenge2("---\n17,x,13,19")?, 3417);
        assert_eq!(super::challenge2(INPUT)?, 1068781);
        assert_eq!(super::challenge2("---\n67,7,59,61")?, 754018);
        assert_eq!(super::challenge2("---\n67,x,7,59,61")?, 779210);
        assert_eq!(super::challenge2("---\n67,7,x,59,61")?, 1261476);
        assert_eq!(super::challenge2("---\n1789,37,47,1889")?, 1202161486);
        Ok(())
    }
}
