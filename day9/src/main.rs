fn main() -> Result<(), day9::Errors> {
    let input = include_str!("input.txt");
    println!("Challenge 1: {}", day9::challenge1(input, 25)?);

    println!("Challenge 2: {}", day9::challenge2(input, 25)?);

    Ok(())
}
