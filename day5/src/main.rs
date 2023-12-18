use day5::Range;
use nom::{
    bytes::complete::tag, character::complete::digit1, character::complete::multispace1,
    error::Error, multi::separated_list1, IResult,
};
use std::collections::HashMap;
use std::env;
use std::fs;

fn parse_numbers(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(multispace1, digit1)(input)
}

fn parse_seed_numbers(input: &str) -> IResult<&str, Vec<&str>> {
    let (numbers, _) = tag::<&str, &str, Error<_>>("seeds:")(input)?;
    let (numbers, _) = multispace1::<&str, Error<_>>(numbers)?;
    parse_numbers(numbers)
}

fn process_ranges(range_lines: Vec<&str>, seed_map: &mut HashMap<u32, u32>) {
    for map_line in range_lines {
        let (_, range_numbers) = parse_numbers(map_line).unwrap();
        let range_numbers: Vec<u32> = range_numbers
            .iter()
            .map(|num| num.parse::<u32>().unwrap())
            .collect();
        let range = Range::new(range_numbers[0], range_numbers[1], range_numbers[2]);
        let seeds: Vec<u32> = seed_map.keys().cloned().collect();
        for seed in seeds {
            match range.map(seed) {
                None => {}
                Some(value) => {
                    seed_map.insert(seed, value);
                }
            }
        }
    }
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
    let mut lines = contents.lines();
    let seed_line = lines.next().unwrap();
    let (_, seed_numbers) = parse_seed_numbers(seed_line).unwrap();
    let seed_numbers: Vec<u32> = seed_numbers
        .iter()
        .map(|num| num.parse::<u32>().unwrap())
        .collect();
    let mut seed_map: HashMap<u32, u32> = HashMap::new();

    for number in &seed_numbers {
        seed_map.insert(*number, *number);
    }

    while let Some(line) = lines.next() {
        match line {
            "seed-to-soil map:"
            | "soil-to-fertilizer map:"
            | "fertilizer-to-water map:"
            | "water-to-light map:"
            | "light-to-temperature map:"
            | "temperature-to-humidity map:"
            | "humidity-to-location map:" => {
                println!("{:?}", line);
                let mut range_lines: Vec<&str> = Vec::new();
                loop {
                    let map_line = lines.next();
                    match map_line {
                        Some("") =>{
                            break;
                        },
                        Some(value) => {
                            range_lines.push(value)
                        },
                        _ => {break}
                    }
                    
                }
                process_ranges(range_lines, &mut seed_map);
                println!("{:?}",seed_map);
            },
            _=>{}
        }
    }
    println!("{:?}", seed_map);
}
