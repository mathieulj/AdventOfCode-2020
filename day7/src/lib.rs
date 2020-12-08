use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn challenge1(input: &str) -> usize {
    let re = Regex::new(r"[[:digit:]]+ (?P<color>[[:alpha:]]+ [[:alpha:]]+) bag").unwrap();

    let containing: HashMap<&str, HashSet<&str>> = input
        .lines()
        .flat_map(
            |line| match line.split(" bags contain ").collect::<Vec<_>>()[..] {
                [bag, contains] => re
                    .captures_iter(contains)
                    .map(move |captures| (captures.name("color").unwrap().as_str(), bag)),
                _ => panic!("Line doesnt match the expected pattern {}", line),
            },
        )
        .fold(HashMap::new(), |mut acc, (contained, contained_by)| {
            acc.entry(contained)
                .or_insert_with(HashSet::new)
                .insert(contained_by);
            acc
        });

    recursive_container(&containing, "shiny gold").len()
}

fn recursive_container<'m>(
    map: &'m HashMap<&str, HashSet<&str>>,
    target: &str,
) -> HashSet<&'m str> {
    if let Some(bags) = map.get(target) {
        let mut union = bags.clone();
        for bag in bags.iter() {
            union.extend(recursive_container(map, bag));
        }
        union
    } else {
        HashSet::new()
    }
}

pub fn challenge2(input: &str) -> u32 {
    let re =
        Regex::new(r"(?P<count>[[:digit:]]+) (?P<color>[[:alpha:]]+ [[:alpha:]]+) bag").unwrap();

    let container: HashMap<&str, HashMap<&str, u32>> = input
        .lines()
        .map(
            |line| match line.split(" bags contain ").collect::<Vec<_>>()[..] {
                [bag, contains] => (
                    bag,
                    re.captures_iter(contains)
                        .map(|captures| {
                            (
                                captures.name("color").unwrap().as_str(),
                                captures.name("count").unwrap().as_str().parse().unwrap(),
                            )
                        })
                        .collect(),
                ),
                _ => panic!("Line doesn't match the expected pattern {}", line),
            },
        )
        .collect();

    recursive_containing(&container, "shiny gold", 1)
}

fn recursive_containing<'m>(
    map: &'m HashMap<&str, HashMap<&str, u32>>,
    target: & str,
    factor: u32,
) -> u32 {
    if let Some(bags) = map.get(target) {
        let mut total_count = 0;
        
        for (bag, count) in bags.iter() {
            total_count += count * factor + recursive_containing(map, bag, factor * count);
        }
        total_count
    } else {
        panic!("Unknown bag {}", target);
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_challenge1() {
        const INPUT: &str = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;

        assert_eq!(super::challenge1(INPUT), 4);
    }

    #[test]
    fn test_challenge2() {
        const INPUT: &str = r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."#;
        assert_eq!(super::challenge2(INPUT), 126);
    }
}
