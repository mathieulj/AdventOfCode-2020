use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let input = include_str!("input.txt");
    println!("Challenge 1: {}", day17::challenge1(input)?);

    println!("Challenge 2: {}", day17::challenge2(input)?);

    Ok(())
}
