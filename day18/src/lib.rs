use std::borrow::Cow;

use anyhow::Result;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};

pub fn challenge1(input: &str) -> Result<u64> {
    Ok(input.lines().map(evaluate_naive).sum())
}

pub fn challenge2(input: &str) -> Result<u64> {
    Ok(input.lines().map(evaluate_naive2).sum())
}

const RE_PAREN_EXPR: Lazy<Regex> = Lazy::new(|| Regex::new("\\(([^()]+)\\)").unwrap());
const RE_GENERIC_EXPR: Lazy<Regex> = Lazy::new(|| Regex::new("^(\\d+) ([+*]) (\\d+)").unwrap());
const RE_ADD_EXPR: Lazy<Regex> = Lazy::new(|| Regex::new("(\\d+) \\+ (\\d+)").unwrap());
const RE_MUL_EXPR: Lazy<Regex> = Lazy::new(|| Regex::new("(\\d+) \\* (\\d+)").unwrap());

pub fn evaluate_naive2(expr: &str) -> u64 {
    let mut expr = Cow::Borrowed(expr);

    while let Cow::Owned(str) = RE_PAREN_EXPR.replace(expr.as_ref(), |captures: &Captures| {
        evaluate_naive2(&captures[1]).to_string()
    }) {
        expr = Cow::Owned(str);
    }

    while let Cow::Owned(str) = RE_ADD_EXPR.replace(expr.as_ref(), |captures: &Captures| {
        let a: u64 = captures[1].parse().expect("Unable to parse a.");
        let b: u64 = captures[2].parse().expect("Unable to parse b.");

        (a + b).to_string()
    }) {
        expr = Cow::Owned(str);
    }

    while let Cow::Owned(str) = RE_MUL_EXPR.replace(expr.as_ref(), |captures: &Captures| {
        let a: u64 = captures[1].parse().expect("Unable to parse a.");
        let b: u64 = captures[2].parse().expect("Unable to parse b.");

        (a * b).to_string()
    }) {
        expr = Cow::Owned(str);
    }

    expr.parse()
        .expect("Expr had something left in it that was not a number")
}

pub fn evaluate_naive(expr: &str) -> u64 {
    let mut expr = Cow::Borrowed(expr);

    while let Cow::Owned(str) = RE_PAREN_EXPR.replace(expr.as_ref(), |captures: &Captures| {
        evaluate_naive(&captures[1]).to_string()
    }) {
        expr = Cow::Owned(str);
    }

    while let Cow::Owned(str) = RE_GENERIC_EXPR.replace(expr.as_ref(), |captures: &Captures| {
        let a: u64 = captures[1].parse().expect("Unable to parse a.");
        let b: u64 = captures[3].parse().expect("Unable to parse b.");

        match &captures[2] {
            "+" => (a + b).to_string(),
            "*" => (a * b).to_string(),
            op => panic!("Unknown op {}", op),
        }
    }) {
        expr = Cow::Owned(str);
    }

    expr.parse()
        .expect("Expr had something left in it that was not a number")
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    const INPUT: &str = r#"1 + 2 * 3 + 4 * 5 + 6
1 + (2 * 3) + (4 * (5 + 6))
2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"#;

    #[test]
    fn test_eval() -> Result<()> {
        use super::evaluate_naive as eval;
        assert_eq!(eval("1 + 2 * 3 + 4 * 5 + 6"), 71);
        assert_eq!(eval("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(eval("2 * 3 + (4 * 5)"), 26);
        assert_eq!(eval("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
        assert_eq!(
            eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
        Ok(())
    }

    #[test]
    fn test_eval2() -> Result<()> {
        use super::evaluate_naive2 as eval;
        assert_eq!(eval("1 + 2 * 3 + 4 * 5 + 6"), 231);
        assert_eq!(eval("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(eval("2 * 3 + (4 * 5)"), 46);
        assert_eq!(eval("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        assert_eq!(eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
        assert_eq!(
            eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            23340
        );
        Ok(())
    }

    #[test]
    fn test_challenge1() -> Result<()> {
        assert_eq!(
            super::challenge1(INPUT)?,
            71 + 51 + 26 + 437 + 12240 + 13632
        );
        Ok(())
    }

    #[test]
    fn test_challenge2() -> Result<()> {
        assert_eq!(
            super::challenge2(INPUT)?,
            231 + 51 + 46 + 1445 + 669060 + 23340
        );
        Ok(())
    }
}
