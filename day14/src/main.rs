fn main() -> Result<(), day14::Errors> {
    let input = include_str!("input.txt");
    println!("Challenge 1: {}", day14::challenge1(input)?);

    println!("Challenge 2: {}", day14::challenge2(input)?);

    Ok(())
}
