#![allow(unstable_name_collisions)]
use displaydoc::Display;
use std::{
    f64::consts::PI,
    ops::{AddAssign, Mul, MulAssign},
};
use thiserror::Error;

#[derive(Debug, Error, Display)]
pub enum Errors {
    /// Parse float error {0}
    ParseFloatError(#[from] std::num::ParseFloatError),
    /// No solution was found
    NoSolutionFound,
    /// Could not interpret the line "{0}"
    BadInputLine(String),
}

/// Fixed point position, will only be accurate if the angle is exclusively on 90deg increments
#[derive(Debug, Copy, Clone, Default)]
struct Position {
    /// Negative represents west
    east: f64,
    /// Negative represents south
    north: f64,
    /// Angle in degrees
    angle: f64,
}

/// Move ignoring the facing direction
#[derive(Debug, Copy, Clone)]
struct Displacement {
    /// Negative represents west
    east: f64,
    /// Negative represents south
    north: f64,
}

impl Displacement {
    fn north(north: f64) -> Self {
        Self { north, east: 0.0 }
    }
    fn south(south: f64) -> Self {
        let north = -south;
        Self { north, east: 0.0 }
    }
    fn east(east: f64) -> Self {
        Self { north: 0.0, east }
    }
    fn west(west: f64) -> Self {
        let east = -west;
        Self { north: 0.0, east }
    }
}

/// Move in the facing direction
struct Forward(f64);

/// Rotation in radians
struct Rotation(f64);

impl Rotation {
    fn from_deg(deg: f64) -> Self {
        Self(PI * deg / 180.0)
    }
}

impl AddAssign<Displacement> for Position {
    fn add_assign(&mut self, rhs: Displacement) {
        self.east += rhs.east;
        self.north += rhs.north;
    }
}

impl AddAssign<Displacement> for Displacement {
    fn add_assign(&mut self, rhs: Displacement) {
        self.east += rhs.east;
        self.north += rhs.north;
    }
}

impl AddAssign<Rotation> for Position {
    fn add_assign(&mut self, rhs: Rotation) {
        self.angle += rhs.0;
    }
}

impl MulAssign<Rotation> for Displacement {
    fn mul_assign(&mut self, rhs: Rotation) {
        let Displacement { east, north } = self.clone();
        let sin = rhs.0.sin();
        let cos = rhs.0.cos();

        self.east = north * sin + east * cos;
        self.north = north * cos - east * sin;
    }
}

impl Mul<f64> for Displacement {
    type Output = Displacement;

    fn mul(self, rhs: f64) -> Self::Output {
        Displacement {
            east: self.east * rhs,
            north: self.north * rhs,
        }
    }
}

impl AddAssign<Forward> for Position {
    fn add_assign(&mut self, rhs: Forward) {
        *self += Displacement {
            east: self.angle.cos() * rhs.0,
            north: -self.angle.sin() * rhs.0,
        }
    }
}

pub fn challenge1(input: &str) -> Result<f64, Errors> {
    let mut position = Position::default();

    for line in input.lines() {
        let magnitude = line[1..].parse()?;

        match &line[0..1] {
            "R" => position += Rotation::from_deg(magnitude),
            "L" => position += Rotation::from_deg(-magnitude),
            "N" => position += Displacement::north(magnitude),
            "S" => position += Displacement::south(magnitude),
            "E" => position += Displacement::east(magnitude),
            "W" => position += Displacement::west(magnitude),
            "F" => position += Forward(magnitude),
            _ => return Err(Errors::BadInputLine(line.to_string())),
        }
    }

    Ok((position.east.abs() + position.north.abs()).round())
}

pub fn challenge2(input: &str) -> Result<f64, Errors> {
    let mut position = Position::default();
    let mut waypoint = Displacement {
        east: 10.0,
        north: 1.0,
    };

    for line in input.lines() {
        let magnitude = line[1..].parse()?;

        match &line[0..1] {
            "R" => waypoint *= Rotation::from_deg(magnitude),
            "L" => waypoint *= Rotation::from_deg(-magnitude),
            "N" => waypoint += Displacement::north(magnitude),
            "S" => waypoint += Displacement::south(magnitude),
            "E" => waypoint += Displacement::east(magnitude),
            "W" => waypoint += Displacement::west(magnitude),
            "F" => position += waypoint * magnitude,
            _ => return Err(Errors::BadInputLine(line.to_string())),
        }
    }

    Ok((position.east.abs() + position.north.abs()).round())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"F10
N3
F7
R90
F11"#;

    #[test]
    fn test_challenge1() -> Result<(), super::Errors> {
        assert_eq!(super::challenge1(INPUT)?, 25.0);
        Ok(())
    }

    #[test]
    fn test_challenge2() -> Result<(), super::Errors> {
        assert_eq!(super::challenge2(INPUT)?, 286.0);
        Ok(())
    }
}
