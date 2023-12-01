use std::path::Path;

use clap::Parser;
use regex::Regex;

#[derive(Parser)]
struct Args {
    filename: String,
}

fn replace_and_parse_lazily(value: &str) -> usize {
    let forward = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();

    let first = forward
        .find(value)
        .unwrap()
        .as_str()
        .replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9")
        .parse::<usize>()
        .unwrap();

    let backward = Regex::new(r"(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|\d)").unwrap();

    let tmp = value.chars().rev().collect::<String>();
    let rev: &str = tmp.as_str();
    let last = backward
        .find(rev)
        .unwrap()
        .as_str()
        .replace("eno", "1")
        .replace("owt", "2")
        .replace("eerht", "3")
        .replace("ruof", "4")
        .replace("evif", "5")
        .replace("xis", "6")
        .replace("neves", "7")
        .replace("thgie", "8")
        .replace("enin", "9")
        .parse::<usize>()
        .unwrap();

    10 * first + last
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let input = std::fs::read_to_string(Path::new(&args.filename))?;

    let sum: usize = input.lines().map(replace_and_parse_lazily).sum();

    println!("{}", sum);

    Ok(())
}
