fn main() -> Result<(), day8::Errors> {
    let input = include_str!("input.txt");
    println!("Challenge 1: {}", day8::challenge1(input)?);

    println!("Challenge 2: {}", day8::challenge2(input)?);

    Ok(())
}
