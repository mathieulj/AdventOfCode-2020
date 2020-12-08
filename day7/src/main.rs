fn main() -> Result<(), day7::Errors> {
    let input = include_str!("input.txt");
    println!("Challenge 1: {}", day7::challenge1(input)?);

    println!("Challenge 2: {}", day7::challenge2(input)?);

    Ok(())
}
