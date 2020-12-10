fn main() -> Result<(), day10::Errors> {
    let input = include_str!("input.txt");
    println!("Challenge 1: {}", day10::challenge1(input)?);

    println!("Challenge 2: {}", day10::challenge2(input)?);

    Ok(())
}
