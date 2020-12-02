use anyhow::Result;
use regex::Regex;

pub fn challenge1(input: &str) -> Result<usize> {
    let pattern = Regex::new("^([[:digit:]]+)-([[:digit:]]+) ([[:alpha:]]): ([[:alpha:]]+)$")?;

    let mut valid_count = 0;
    for line in input.lines() {
        let matches = pattern
            .captures(line)
            .ok_or_else(|| anyhow::anyhow!("Line didn't match the regex"))?;

        let min: usize = matches[1].parse()?;
        let max: usize = matches[2].parse()?;
        let expected = &matches[3];
        let password = &matches[4];

        if password_valid_policy1(min, max, expected, password) {
            valid_count += 1;
        }
    }

    Ok(valid_count)
}

fn password_valid_policy1(min: usize, max: usize, expected: &str, password: &str) -> bool {
    let occurrences = password.split(expected).count() - 1;
    min <= occurrences && occurrences <= max
}

pub fn challenge2(input: &str) -> Result<usize> {
    let pattern = Regex::new("^([[:digit:]]+)-([[:digit:]]+) ([[:alpha:]]): ([[:alpha:]]+)$")?;

    let mut valid_count = 0;
    for line in input.lines() {
        let matches = pattern
            .captures(line)
            .ok_or_else(|| anyhow::anyhow!("Line didn't match the regex"))?;

        let a: usize = matches[1].parse()?;
        let b: usize = matches[2].parse()?;
        let expected = &matches[3];
        let haystack = &matches[4];

        if password_valid_policy2(a, b, expected, haystack)? {
            valid_count += 1;
        }
    }

    Ok(valid_count)
}

fn password_valid_policy2(ia: usize, ib: usize, expected: &str, password: &str) -> Result<bool> {
    let expected = expected
        .chars()
        .next()
        .ok_or_else(|| anyhow::anyhow!("Empty condition"))?;

    let a = password.chars().nth(ia - 1);
    let b = password.chars().nth(ib - 1);

    Ok(match (a, b) {
        (Some(a), Some(b)) if a == expected && b == expected => false,
        (Some(a), ..) if a == expected => true,
        (.., Some(b)) if b == expected => true,
        _ => false,
    })
}

#[cfg(test)]
mod tests {
    use anyhow::{ensure, Result};
    const INPUT: &str = r#"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"#;

    #[test]
    fn challenge1() -> Result<()> {
        let count = super::challenge1(INPUT)?;
        ensure!(count == 2, "Wrong number of valid reported ({})", count);

        Ok(())
    }

    #[test]
    fn challenge2() -> Result<()> {
        let count = super::challenge2(INPUT)?;
        ensure!(count == 1, "Wrong number of valid reported ({})", count);

        Ok(())
    }
}
