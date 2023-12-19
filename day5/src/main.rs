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

fn process_ranges(range_lines: Vec<&str>, seed_map: &mut HashMap<u64, u64>) {
    let seeds: Vec<u64> = seed_map.keys().copied().collect();
    for seed  in seeds {
        for map_line in &range_lines {
        let (_, range_numbers) = parse_numbers(map_line).unwrap();
        let range_numbers: Vec<u64> = range_numbers
            .iter()
            .map(|num| num.parse::<u64>().unwrap())
            .collect();
        let range = Range::new(range_numbers[0], range_numbers[1], range_numbers[2]);
            match range.translate_single(*seed_map.get(&seed).unwrap()) {
                None => {},
                Some(value) => {
                    seed_map.insert(seed, value);
                    break;
                }
            }

    }}
}

fn part1(contents: &String){
    let mut lines = contents.lines();
    let seed_line = lines.next().unwrap();
    let (_, seed_numbers) = parse_seed_numbers(seed_line).unwrap();
    let seed_numbers: Vec<u64> = seed_numbers
        .iter()
        .map(|num| num.parse::<u64>().unwrap())
        .collect();
    let mut seed_map: HashMap<u64, u64> = HashMap::new();
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
            },
            _=>{}
        }
    }
    let part1_result = seed_map.values().min().unwrap();
    println!("lowest location: {}",part1_result);
}

fn part2(contents: &String){
    let mut lines = contents.lines();
    let seed_line = lines.next().unwrap();
    let (_, seed_numbers) = parse_seed_numbers(seed_line).unwrap();
    let seed_numbers: Vec<u64> = seed_numbers
        .iter()
        .map(|num| num.parse::<u64>().unwrap())
        .collect();
    println!("{:?}",seed_numbers);
}




fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &str = match args.len() {
        2 => {
            println!("Finding solution for  {}", args[1]);
            &args[1]
        }
        _ => panic!("only one argument is needed"),
    };
    let contents = fs::read_to_string(file_path).expect("File does not exist");
    part1(&contents);
    part2(&contents);
}
