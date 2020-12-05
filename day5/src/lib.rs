use std::collections::BTreeSet;

pub fn challenge1(input: &str) -> u16 {
    input.lines().map(parse_seat_id).max().unwrap_or(0)
}

pub fn challenge2(input: &str) -> u16 {
    let assignments: BTreeSet<u16> = input.lines().map(parse_seat_id).collect();

    let mut last_seat = assignments.iter().next().copied().unwrap_or(0);
    for &seat in assignments.iter().skip(1) {
        if seat == last_seat + 2 {
            break;
        } else {
            last_seat = seat;
        }
    }

    last_seat + 1
}

pub fn parse_seat_id(line: &str) -> u16 {
    line.chars().fold(0, |acc, c| match c {
        'B' => (acc << 1) | 1,
        'R' => (acc << 1) | 1,
        _ => acc << 1,
    })
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL"#;

    #[test]
    fn test_seat_id() {
        assert_eq!(super::parse_seat_id("FBFBBFFRLR"), 357);
        assert_eq!(super::parse_seat_id("BFFFBBFRRR"), 567);
        assert_eq!(super::parse_seat_id("FFFBBBFRRR"), 119);
        assert_eq!(super::parse_seat_id("BBFFBBFRLL"), 820);
    }

    #[test]
    fn test_challenge1() {
        assert_eq!(super::challenge1(INPUT), 820);
    }
}
