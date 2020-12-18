use itertools::Itertools as _;
use std::collections::HashSet;
use std::num::ParseIntError;

#[derive(Debug, Default, Clone, Copy)]
struct Bound {
    min: isize,
    max: isize,
}

impl Bound {
    fn extend(&mut self, value: isize) {
        if self.min > value {
            self.min = value;
        } else if self.max < value {
            self.max = value;
        }
    }
}

type Coordinate = Vec<isize>;

fn active_neighbor_count(coordinate: &[isize], active: &HashSet<Coordinate>) -> usize {
    coordinate
        .iter()
        .map(|&b| b - 1..=b + 1)
        .multi_cartesian_product()
        .filter(|p| &p[..] != coordinate)
        .filter(|p| active.contains(p))
        .count()
}

fn parse(
    input: &str,
    dimensions: usize,
) -> Result<(Vec<Bound>, HashSet<Coordinate>), ParseIntError> {
    let mut bounds = vec![Bound::default(); dimensions];

    let active: HashSet<Coordinate> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(move |(x, _)| vec![x as isize, y as isize])
        })
        .map(|mut v| {
            v.iter()
                .zip(bounds.iter_mut())
                .for_each(|(value, bound)| bound.extend(*value));

            v.extend(std::iter::repeat(0).take(bounds.len() - v.len()));
            v
        })
        .collect();

    Ok((bounds, active))
}

fn simulate_cycle(bounds: &mut Vec<Bound>, active: &mut HashSet<Vec<isize>>) {
    let inactivated: Vec<Coordinate> = active
        .iter()
        .filter(|&position| !matches!(active_neighbor_count(position, &active), 2 | 3))
        .cloned()
        .collect();

    let activated: Vec<Coordinate> = bounds
        .iter()
        .map(|b| b.min - 1..=b.max + 1)
        .multi_cartesian_product()
        .filter(|position| !active.contains(position))
        .filter(|position| active_neighbor_count(position, &active) == 3)
        .collect();

    active.extend(activated.into_iter().inspect(|v| {
        v.iter()
            .zip(bounds.iter_mut())
            .for_each(|(value, bound)| bound.extend(*value));
    }));

    for coordinates in inactivated.into_iter() {
        active.remove(&coordinates);
    }
}

pub fn challenge1(input: &str) -> Result<usize, ParseIntError> {
    let (mut bounds, mut active) = parse(input, 3)?;

    for _ in 0..6 {
        simulate_cycle(&mut bounds, &mut active);
    }

    Ok(active.len())
}

pub fn challenge2(input: &str) -> Result<usize, ParseIntError> {
    let (mut bounds, mut active) = parse(input, 4)?;

    for _ in 0..6 {
        simulate_cycle(&mut bounds, &mut active);
    }

    Ok(active.len())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#".#.
..#
###"#;

    #[test]
    fn test_challenge1() -> Result<(), super::ParseIntError> {
        assert_eq!(super::challenge1(INPUT)?, 112);
        Ok(())
    }

    #[test]
    fn test_challenge2() -> Result<(), super::ParseIntError> {
        assert_eq!(super::challenge2(INPUT)?, 848);
        Ok(())
    }
}
