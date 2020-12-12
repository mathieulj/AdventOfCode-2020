#![allow(unstable_name_collisions)]
use displaydoc::Display;
use itertools::Itertools as _;
use std::convert::{TryFrom, TryInto};
use thiserror::Error;
use utils::BoolExt as _;

#[derive(Debug, Error, Display)]
pub enum Errors {
    /// No solution was found
    NoSolutionFound,
    /// Invalid character {0} in input.
    InvalidCharacter(char),
    /// An input row is of different length then the others
    JaggedInput,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Spot {
    EmptySeat,
    FilledSeat,
    Isle,
}

impl TryFrom<char> for Spot {
    type Error = Errors;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Spot::Isle),
            '#' => Ok(Spot::FilledSeat),
            'L' => Ok(Spot::EmptySeat),
            _ => Err(Errors::InvalidCharacter(value)),
        }
    }
}

/// Add a usize to an isize safely, returns None if the result would be invalid
fn checked_add(a: usize, b: isize) -> Option<usize> {
    if b.is_negative() {
        a.checked_sub(b.checked_neg()?.try_into().ok()?)
    } else {
        a.checked_add(b.try_into().ok()?)
    }
}

/// Counts the number of neighboring seats that are occupied
fn neighboring_occupied(plane: &Vec<Vec<Spot>>, pos: (usize, usize)) -> usize {
    [
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
    ]
    .iter()
    .filter_map(|&direction| {
        let row = plane.get(checked_add(pos.0, direction.0)?)?;
        let spot = row.get(checked_add(pos.1, direction.1)?)?;

        (*spot == Spot::FilledSeat).then(|| ())
    })
    .count()
}

/// Counts the number of visible seats that are occupied
fn visible_occupied(plane: &Vec<Vec<Spot>>, pos: (usize, usize)) -> usize {
    [
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
    ]
    .iter()
    .filter(|&&direction| see_occupied(plane, pos, direction))
    .count()
}

/// Check if the next visible seat is occupied
fn see_occupied(plane: &Vec<Vec<Spot>>, pos: (usize, usize), direction: (isize, isize)) -> bool {
    checked_add(pos.0, direction.0)
        .zip(checked_add(pos.1, direction.1))
        .map(
            |next| match plane.get(next.0).and_then(|row| row.get(next.1)) {
                Some(Spot::FilledSeat) => true,
                Some(Spot::Isle) => see_occupied(plane, next, direction),
                Some(Spot::EmptySeat) => false,
                None => false,
            },
        )
        .unwrap_or(false)
}

/// Traverse the 2D map yielding cells and their positions
fn traverse(plane: &Vec<Vec<Spot>>) -> impl Iterator<Item = (&Spot, (usize, usize))> {
    plane.iter().enumerate().flat_map(|(row_i, row)| {
        row.iter()
            .enumerate()
            .map(move |(col_i, spot)| (spot, (row_i, col_i)))
    })
}

fn parse_layout(input: &str) -> Result<Vec<Vec<Spot>>, Errors> {
    input
        .lines()
        .map(|line| line.chars().map(Spot::try_from).try_collect())
        .try_collect()
}

pub fn challenge1(input: &str) -> Result<usize, Errors> {
    let mut current = parse_layout(input)?;
    let width = current.first().ok_or(Errors::NoSolutionFound)?.len();
    if current.iter().any(|row| row.len() != width) {
        return Err(Errors::JaggedInput);
    }

    let mut previous = current.clone();

    loop {
        std::mem::swap(&mut current, &mut previous);

        for (&spot, position) in traverse(&previous) {
            current[position.0][position.1] =
                match (spot, neighboring_occupied(&previous, position)) {
                    (Spot::FilledSeat, 4..=8) => Spot::EmptySeat,
                    (Spot::EmptySeat, 0) => Spot::FilledSeat,
                    (spot, _) => spot,
                };
        }

        if current == previous {
            let occupied_count = current
                .iter()
                .flat_map(|r| r.iter())
                .filter(|&&s| s == Spot::FilledSeat)
                .count();

            return Ok(occupied_count);
        }
    }
}

pub fn challenge2(input: &str) -> Result<usize, Errors> {
    let mut current = parse_layout(input)?;
    let width = current.first().ok_or(Errors::NoSolutionFound)?.len();
    if current.iter().any(|row| row.len() != width) {
        return Err(Errors::JaggedInput);
    }

    let mut previous = current.clone();

    loop {
        std::mem::swap(&mut current, &mut previous);

        for (&spot, position) in traverse(&previous) {
            current[position.0][position.1] = match (spot, visible_occupied(&previous, position)) {
                (Spot::FilledSeat, 5..=8) => Spot::EmptySeat,
                (Spot::EmptySeat, 0) => Spot::FilledSeat,
                (spot, _) => spot,
            };
        }

        if current == previous {
            let occupied_count = current
                .iter()
                .flat_map(|r| r.iter())
                .filter(|&&s| s == Spot::FilledSeat)
                .count();

            return Ok(occupied_count);
        }
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"#;

    #[test]
    fn test_challenge1() -> Result<(), super::Errors> {
        assert_eq!(super::challenge1(INPUT)?, 37);
        Ok(())
    }

    #[test]
    fn test_challenge2() -> Result<(), super::Errors> {
        assert_eq!(super::challenge2(INPUT)?, 26);
        Ok(())
    }
}
