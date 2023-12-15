use nom::{
    bytes::complete::tag, character::complete::digit1, character::complete::multispace1,
    error::Error, multi::separated_list1, sequence::separated_pair, IResult,
};
use std::collections::HashMap;
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

fn parse_card(input: &str, card_map: &mut HashMap<u32, u32>) -> u32 {
    let (rest, (_, card_num)) = parse_card_label(input).unwrap();
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

    // part 1 specific
    let mut result_part1 = 0;
    let mut result_part2 = 0;
    for num in &numbers_in_hand {
        if winning_numbers.contains(&num) {
            if result_part1 == 0 {
                result_part1 = 1;
            } else {
                result_part1 *= 2;
            }
            result_part2 += 1
        }
    }

    // part 2 specific
    let card_number = card_num.parse::<u32>().unwrap();

    // add original
    card_map
        .entry(card_number)
        .and_modify(|e| *e += 1)
        .or_insert(1);

    if result_part2 > 0 {
        let update_num = *card_map.get(&card_number).unwrap();
        for won_card in card_number + 1..card_number + result_part2 + 1 {
            card_map
                .entry(won_card)
                .and_modify(|e| *e += update_num)
                .or_insert(update_num);
        }
    }

    result_part1
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
    let mut card_map: HashMap<u32, u32> = HashMap::new();
    let result1: u32 = contents.lines().map(|s| parse_card(s, &mut card_map)).sum();

    // hash map of card number the value of the win and how many copies
    let result2: u32 = card_map.values().sum();
    println!("part 1: {:?} part 2: {:?}", result1, result2);
}
