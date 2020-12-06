use std::collections::HashSet;
use std::iter;

pub fn challenge1(input: &str) -> usize {
    let mut census = Vec::new();
    input
        .lines()
        .chain(iter::once(""))
        .fold(HashSet::new(), |mut answers, line| {
            if line.is_empty() {
                census.push(answers.len());
                HashSet::new()
            } else {
                answers.extend(line.chars());
                answers
            }
        });

    census.iter().sum()
}

pub fn challenge2(input: &str) -> usize {
    let mut census = Vec::new();
    input
        .lines()
        .chain(iter::once(""))
        .map(|line| line.chars().collect::<HashSet<char>>())
        .fold(
            None,
            |previous_answers: Option<HashSet<char>>, line_answers| match previous_answers {
                Some(previous_answers) if line_answers.is_empty() => {
                    census.push(previous_answers.len());
                    None
                }
                Some(previous_answers) => Some(
                    line_answers
                        .intersection(&previous_answers)
                        .copied()
                        .collect(),
                ),
                None if line_answers.is_empty() => None,
                None => Some(line_answers),
            },
        );

    census.iter().sum()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;

    #[test]
    fn test_challenge1() {
        assert_eq!(super::challenge1(INPUT), 11);
    }

    #[test]
    fn test_challenge2() {
        assert_eq!(super::challenge2(INPUT), 6);
    }
}
