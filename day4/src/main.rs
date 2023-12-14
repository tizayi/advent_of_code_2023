use nom::{
    bytes::complete::tag, character::complete::digit1, character::complete::multispace1,
    error::Error, multi::separated_list1, sequence::separated_pair, IResult,
};
use std::env;
use std::fs;

fn parse_card_label(input: &str) -> IResult<&str, (&str, &str)> {
    Ok(separated_pair(tag("Card"), multispace1, digit1)(input)?)
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(multispace1, digit1)(input)
}

fn seperator(input: &str) -> IResult<&str, &str> {
    let (rest, _) = tag::<&str, &str, Error<_>>(" |")(input)?;
    multispace1::<&str, Error<_>>(rest)
}

fn parse_card(input: &str) -> u32 {
    let (rest, _) = parse_card_label(input).unwrap();
    let (numbers, _) = tag::<&str, &str, Error<_>>(":")(rest).unwrap();
    let (numbers, _) = multispace1::<&str, Error<_>>(numbers).unwrap();
    let (_, (winning_str, in_hand_str)) =
        separated_pair(parse_numbers, seperator, parse_numbers)(numbers).unwrap();

    let winning_numbers: Vec<u32> = winning_str
        .iter()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let numbers_in_hand: Vec<u32> = in_hand_str
        .iter()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let mut result = 0;
    for num in &numbers_in_hand {
        if winning_numbers.contains(&num) {
            if result == 0 {
                result = 1;
            } else {
                result *= 2;
            }
        }
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &str = match args.len() {
        2 => {
            println!("Finding calibration in {}", args[1]);
            &args[1]
        }
        _ => panic!("only one argument is needed"),
    };
    let contents = fs::read_to_string(file_path).expect("File does not exist");
    let result1: u32 = contents.lines().map(|s| parse_card(s)).sum();
    println!("part1: {:?}", result1)
}
