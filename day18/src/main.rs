use anyhow::Result;
fn main() -> Result<()> {
    let input = include_str!("input.txt");
    println!("Challenge 1: {}", day18::challenge1(input)?);

    println!("Challenge 2: {}", day18::challenge2(input)?);

    Ok(())
}
