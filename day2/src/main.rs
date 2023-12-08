use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map, error::Error,
    multi::separated_list0, sequence::separated_pair, IResult,
};
use std::collections::HashMap;
use std::env;
use std::fs;

fn parse_balls(input: &str) -> IResult<&str, (&str, &str)> {
    Ok(separated_pair(digit1, tag(" "), parse_colour)(input)?)
}

fn parse_hand(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    separated_list0(tag(", "), parse_balls)(input)
}

fn parse_colour(input: &str) -> IResult<&str, &str> {
    alt((tag("blue"), tag("green"), tag("red")))(input)
}

fn check_hand(hand: Vec<(&str, &str)>, bag: &HashMap<&str, u32>) -> bool {
    for item in hand {
        let num: u32 = item.0.parse().unwrap();
        if num > bag[&item.1] {
            return false;
        }
    }
    true
}

fn parse_game_number(input: &str) -> IResult<&str, &str> {
    let (rest, (_, game)) = separated_pair(tag("Game"), tag(" "), digit1)(input)?;
    Ok((rest, game))
}

fn update_cube_map<'a>(cubes: &Vec<(&str, &'a str)>, cube_map: &mut HashMap<&'a str, Option<u32>>) {
    for item in cubes {
        let num: u32 = item.0.parse().unwrap();
        match cube_map[&item.1] {
            None => {
                cube_map.insert(&item.1, Some(num));
            }
            Some(value) => {
                if value < num {
                    cube_map.insert(&item.1, Some(num));
                }
            }
        };
    }
}

fn parse_game(input: &str, bag: &HashMap<&str, u32>) -> (u32, u32) {
    let (rest, num) = parse_game_number(input).unwrap();
    let game_number: u32 = num.parse().unwrap();
    let (rest, _) = tag::<&str, &str, Error<_>>(": ")(rest).unwrap();
    let hand_works_parser = map(parse_hand, |hand: Vec<(&str, &str)>| check_hand(hand, bag));

    let (_, things) = separated_list0(tag("; "), parse_hand)(rest).unwrap();

    let mut fewest_cubes: HashMap<&str, Option<u32>> =
        [("blue", None), ("red", None), ("green", None)]
            .iter()
            .cloned()
            .collect();

    for thing in things {
        update_cube_map(&thing, &mut fewest_cubes);
    }

    let mut result2: u32 = 1;
    for (_, cube) in fewest_cubes {
        match cube {
            None => {}
            Some(value) => result2 *= value,
        }
    }
    let (_, result) = separated_list0(tag("; "), hand_works_parser)(rest).unwrap();

    if result.iter().all(|x| *x) {
        return (game_number, result2);
    }
    (0, result2)
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
    let bag: HashMap<&str, u32> = [("blue", 14), ("red", 12), ("green", 13)]
        .iter()
        .cloned()
        .collect();

    let results = contents
        .lines()
        .map(|line| parse_game(line, &bag))
        .reduce(|a: (u32, u32), b: (u32, u32)| (a.0 + b.0, a.1 + b.1));
    println!("part1 {}, part2 {}", results.unwrap().0, results.unwrap().1);
}
