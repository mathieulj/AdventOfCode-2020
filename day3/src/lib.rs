#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Slope {
    pub horizontal: usize,
    pub vertical: usize,
}

impl Slope {
    pub fn new(horizontal: usize, vertical: usize) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Location {
    Tree,
    Open,
}

impl From<char> for Location {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Tree,
            _ => Self::Open,
        }
    }
}

pub fn challenge1(input: &str) -> usize {
    count_trees(input, &Slope::new(3, 1))
}

pub fn challenge2(input: &str) -> usize {
    [
        Slope::new(1, 1),
        Slope::new(3, 1),
        Slope::new(5, 1),
        Slope::new(7, 1),
        Slope::new(1, 2),
    ]
    .iter()
    .map(|slope| count_trees(input, slope))
    .product()
}

pub fn count_trees(input: &str, slope: &Slope) -> usize {
    input
        .lines()
        .step_by(slope.vertical)
        .enumerate()
        .filter_map(|(index, line)| line.chars().cycle().step_by(slope.horizontal).nth(index))
        .map(Location::from)
        .filter(|&location| location == Location::Tree)
        .count()
}

#[cfg(test)]
mod tests {
    use crate::Slope;

    const INPUT: &str = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;

    #[test]
    fn challenge1() {
        assert_eq!(super::challenge1(INPUT), 7);
    }
    #[test]
    fn challenge2() {
        assert_eq!(super::challenge2(INPUT), 336);
    }

    #[test]
    fn count_trees() {
        use super::{count_trees, Slope};

        assert_eq!(count_trees(INPUT, &Slope::new(1, 1)), 2);
        assert_eq!(count_trees(INPUT, &Slope::new(3, 1)), 7);
        assert_eq!(count_trees(INPUT, &Slope::new(5, 1)), 3);
        assert_eq!(count_trees(INPUT, &Slope::new(7, 1)), 4);
        assert_eq!(count_trees(INPUT, &Slope::new(1, 2)), 2);
    }
}
