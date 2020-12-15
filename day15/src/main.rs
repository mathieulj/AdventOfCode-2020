fn main() -> Result<(), day15::Errors> {
    let input = include_str!("input.txt");
    println!("Challenge 1: {}", day15::challenge1(input)?);

    println!("Challenge 2: {}", day15::challenge2(input)?);

    Ok(())
}
