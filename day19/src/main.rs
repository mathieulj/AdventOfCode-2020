use anyhow::Result;
fn main() -> Result<()> {
    let input = include_str!("input.txt");
    println!("Challenge 1: {}", day19::challenge1(input)?);

    let input = include_str!("input2.txt");
    println!("Challenge 2: {}", day19::challenge1(input)?);

    Ok(())
}
