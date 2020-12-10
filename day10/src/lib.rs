use displaydoc::Display;
use itertools::Itertools as _;
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

#[derive(Debug, Error, Display)]
pub enum Errors {
    /// Parse error {0}
    ParseError(#[from] std::num::ParseIntError),
    /// No solution found
    NoSolution,
}

pub fn challenge1(input: &str) -> Result<u64, Errors> {
    let mut jolts: BTreeSet<u64> = input.lines().map(str::parse).try_collect()?;
    let max = *jolts.iter().rev().next().ok_or(Errors::NoSolution)?;

    jolts.insert(0);
    jolts.insert(max + 3);

    let (ones, threes): (u64, u64) =
        jolts
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .fold((0, 0), |(ones, threes), diff| match diff {
                1 => (ones + 1, threes),
                3 => (ones, threes + 1),
                _ => (ones, threes),
            });
    Ok(ones * threes)
}

pub fn challenge2(input: &str) -> Result<u64, Errors> {
    let jolts: BTreeSet<u64> = input.lines().map(str::parse).try_collect()?;

    let last = 3 + *jolts.iter().rev().next().ok_or(Errors::NoSolution)?;
    // Map of <next jolt to test> : <number of branches leading to it>
    let mut next: BTreeMap<u64, u64> = (1..=3)
        .filter(|o| jolts.contains(o))
        .map(|i| (i, 1))
        .collect();

    let mut complete_branches = 0;
    while !next.is_empty() {
        // This condition is checked for the while loop so unwrap
        let j = *next.keys().next().unwrap();
        let count = next.remove(&j).unwrap();

        for value in j + 1..=j + 3 {
            if value == last {
                complete_branches += count
            } else if jolts.contains(&value) {
                *next.entry(value).or_insert(0) += count
            }
        }
    }

    Ok(complete_branches)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"16
10
15
5
1
11
7
19
6
12
4"#;

    const INPUT2: &str = r#"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"#;

    #[test]
    fn test_challenge1() -> Result<(), super::Errors> {
        assert_eq!(super::challenge1(INPUT)?, 35);
        assert_eq!(super::challenge1(INPUT2)?, 220);
        Ok(())
    }

    #[test]
    fn test_challenge2() -> Result<(), super::Errors> {
        assert_eq!(super::challenge2(INPUT)?, 8);
        assert_eq!(super::challenge2(INPUT2)?, 19208);
        Ok(())
    }
}
