fn main() -> Result<(), day13::Errors> {
    let input = include_str!("input.txt");
    println!("Challenge 1: {}", day13::challenge1(input)?);

    println!("Challenge 2: {}", day13::challenge2(input)?);

    Ok(())
}
