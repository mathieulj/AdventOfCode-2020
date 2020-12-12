fn main() -> Result<(), day11::Errors> {
    let input = include_str!("input.txt");
    println!("Challenge 1: {}", day11::challenge1(input)?);

    println!("Challenge 2: {}", day11::challenge2(input)?);

    Ok(())
}
