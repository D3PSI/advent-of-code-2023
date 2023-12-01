use std::path::Path;

use clap::Parser;

#[derive(Parser)]
struct Args {
    filename: String,
}

fn digits(val: &str) -> usize {
    let digits: Vec<usize> = val
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let first = digits.first().unwrap_or(&0);
    let last = digits.last().unwrap_or(&0);

    10 * first + last
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let input = std::fs::read_to_string(Path::new(&args.filename))?;

    let sum: usize = input.lines().map(digits).sum();

    println!("{}", sum);

    Ok(())
}
