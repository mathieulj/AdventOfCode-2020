#![allow(unstable_name_collisions)]
use std::collections::HashSet;
use utils::BoolExt as _;

pub fn challenge1(input: &str) -> i64 {
    let mut others = HashSet::new();
    input
        .lines()
        .map(|line| i64::from_str_radix(line, 10).expect("Bad radix value"))
        .find_map(|number| {
            let compliment = 2020 - number;
            others.insert(number);
            others.contains(&compliment).then(|| compliment * number)
        })
        .expect("Nothing matched")
}

pub fn challenge2(input: &str) -> i64 {
    let numbers: HashSet<i64> = input
        .lines()
        .map(|line| i64::from_str_radix(line, 10).expect("Bad radix value"))
        .collect();

    numbers
        .iter()
        .flat_map(|a| numbers.iter().map(move |b| (a, b)))
        .find_map(|(a, b)| {
            let compliment = 2020 - a - b;
            numbers.contains(&compliment).then(|| compliment * a * b)
        })
        .expect("Nothing matched")
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"1721
979
366
299
675
1456"#;

    #[test]
    fn first() {
        assert_eq!(super::challenge1(INPUT), 514579);
    }

    #[test]
    fn second() {
        assert_eq!(super::challenge2(INPUT), 241861950);
    }
}
