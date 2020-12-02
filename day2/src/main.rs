use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    println!("Challenge 1: {}", day2::challenge1(input)?);

    println!("Challenge 2: {}", day2::challenge2(input)?);

    Ok(())
}
