fn main() -> Result<(), day12::Errors> {
    let input = include_str!("input.txt");
    println!("Challenge 1: {}", day12::challenge1(input)?);

    println!("Challenge 2: {}", day12::challenge2(input)?);

    Ok(())
}
