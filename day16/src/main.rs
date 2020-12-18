fn main() -> Result<(), day16::Errors> {
    let input = include_str!("input.txt");
    println!("Challenge 1: {}", day16::challenge1(input)?);

    println!("Challenge 2: {}", day16::challenge2(input, "departure")?);

    Ok(())
}
