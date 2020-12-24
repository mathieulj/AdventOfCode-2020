use std::{collections::HashMap, convert::TryFrom, vec};

use anyhow::Context as _;
use anyhow::Result;
use itertools::Itertools as _;
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, Clone)]
enum RuleSegment<'s> {
    Literal(&'s str),
    Reference(u32),
}

const LITERAL_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^"([[:alpha:]])"$"#).expect("Bad regex"));

impl<'s> TryFrom<&'s str> for RuleSegment<'s> {
    type Error = anyhow::Error;

    fn try_from(value: &'s str) -> Result<Self, Self::Error> {
        if let Some(captures) = LITERAL_RE.captures(value) {
            Ok(RuleSegment::Literal(
                captures
                    .get(1)
                    .context("Bad regex, no capture group")?
                    .as_str(),
            ))
        } else {
            Ok(RuleSegment::Reference(value.parse()?))
        }
    }
}

/// Return possible remaining strings after validating with the given rule.
/// An empty remainder means that the rule matched the entire string
fn valid_remainders<'s, 'r>(
    message: &'s str,
    rule: &Vec<RuleSegment<'s>>,
    rules: &'r HashMap<u32, Vec<Vec<RuleSegment<'s>>>>,
) -> Box<dyn Iterator<Item = &'s str> + 'r> {
    let mut remainders = vec![message];
    for segment in rule {
        remainders = remainders
            .into_iter()
            .flat_map(|remainder| match segment {
                RuleSegment::Literal(l) => {
                    if remainder.len() >= l.len() && *l == &remainder[..l.len()] {
                        Box::new(std::iter::once(&remainder[l.len()..]))
                            as Box<dyn Iterator<Item = &'s str> + 'r>
                    } else {
                        Box::new(std::iter::empty())
                    }
                }
                RuleSegment::Reference(r) => Box::new(
                    rules
                        .get(r)
                        .into_iter()
                        .flat_map(|options| options.iter())
                        .flat_map(move |rule| valid_remainders(remainder, rule, rules)),
                ),
            })
            .collect();
    }

    Box::new(remainders.into_iter())
}

pub fn challenge1(input: &str) -> Result<usize> {
    let (rules, messages): (&str, &str) = input
        .split("\n\n")
        .collect_tuple()
        .context("Input did not have two sections")?;

    let rules: HashMap<u32, Vec<Vec<RuleSegment>>> = rules
        .lines()
        .map(|line| -> Result<(u32, Vec<Vec<RuleSegment>>)> {
            let (id, rules): (&str, &str) = line
                .split(": ")
                .collect_tuple()
                .context("Line did not have two sections")?;

            let alternatives = rules
                .split(" | ")
                .map(|alternative| {
                    alternative
                        .split(" ")
                        .map(RuleSegment::try_from)
                        .try_collect()
                        .context(format!("Parsing rule alternative {}", alternative))
                })
                .try_collect()
                .context("Parsing rule")?;

            Ok((id.parse().context("Parsing rule id")?, alternatives))
        })
        .try_collect()?;

    Ok(messages
        .lines()
        .filter(|message| {
            valid_remainders(message, &rules[&0][0], &rules).any(|remainder| remainder == "")
        })
        .count())
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    const INPUT: &str = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

    #[test]
    fn test_challenge1() -> Result<()> {
        assert_eq!(super::challenge1(INPUT)?, 2);
        Ok(())
    }
}
