use displaydoc::Display;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    convert::TryFrom,
};
use thiserror::Error;

struct Rule<'a> {
    bag: &'a str,
    contents: HashMap<&'a str, u32>,
}

#[derive(Debug, Error, Display)]
pub enum Errors {
    /// Parse error {0}
    ParseError(#[from] std::num::ParseIntError),
    /// Bad line {0}
    BadLine(String),
    /// Bad regex {0}
    BadRegex(#[from] regex::Error),
    /// Missing placeholder {0}
    MissingPlaceholder(&'static str),
    /// Bag bag {0}
    UnknownBag(String),
}

impl<'a> TryFrom<&'a str> for Rule<'a> {
    type Error = Errors;

    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        let re = Regex::new(r"(?P<count>[[:digit:]]+) (?P<color>[[:alpha:]]+ [[:alpha:]]+) bag")?;

        match line.split(" bags contain ").collect::<Vec<_>>()[..] {
            [bag, contains] => {
                let contents = re
                    .captures_iter(contains)
                    .map(|captures| {
                        Ok((
                            captures
                                .name("color")
                                .ok_or(Errors::MissingPlaceholder("color"))?
                                .as_str(),
                            captures
                                .name("count")
                                .ok_or(Errors::MissingPlaceholder("count"))?
                                .as_str()
                                .parse()?,
                        ))
                    })
                    .collect::<Result<_, Errors>>()?;

                Ok(Rule { bag, contents })
            }
            _ => Err(Errors::BadLine(line.to_string())),
        }
    }
}

pub fn challenge1(input: &str) -> Result<usize, Errors> {
    let rules = input
        .lines()
        .map(Rule::try_from)
        .collect::<Result<Vec<Rule>, _>>()?;

    let containing: HashMap<&str, HashSet<&str>> =
        rules.into_iter().fold(HashMap::new(), |mut acc, rule| {
            let Rule { bag, contents } = rule;
            contents.into_iter().for_each(|(contained, _count)| {
                acc.entry(contained)
                    .or_insert_with(HashSet::new)
                    .insert(bag);
            });
            acc
        });

    fn recurse<'m>(map: &'m HashMap<&str, HashSet<&str>>, target: &str) -> HashSet<&'m str> {
        map.get(target)
            .into_iter()
            .flat_map(|bags| {
                bags.iter()
                    .copied()
                    .chain(bags.iter().flat_map(|bag| recurse(map, bag)))
            })
            .collect()
    }

    Ok(recurse(&containing, "shiny gold").len())
}

pub fn challenge2(input: &str) -> Result<u32, Errors> {
    let index: HashMap<&str, Rule> = input
        .lines()
        .map(Rule::try_from)
        .map(|result| result.map(|rule| (rule.bag, rule)))
        .collect::<Result<_, _>>()?;

    fn recurse<'m>(index: &'m HashMap<&str, Rule>, target: &str) -> Result<u32, Errors> {
        let rule = index
            .get(target)
            .ok_or_else(|| Errors::UnknownBag(target.to_string()))?;

        let mut sum = 0;
        for (bag, count) in rule.contents.iter() {
            sum += count + count * recurse(index, bag)?;
        }

        Ok(sum)
    }

    recurse(&index, "shiny gold")
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_challenge1() -> Result<(), super::Errors> {
        const INPUT: &str = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;

        assert_eq!(super::challenge1(INPUT)?, 4);
        Ok(())
    }

    #[test]
    fn test_challenge2() -> Result<(), super::Errors> {
        const INPUT: &str = r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."#;
        assert_eq!(super::challenge2(INPUT)?, 126);
        Ok(())
    }
}
