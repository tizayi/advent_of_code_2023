use day5::Range;
use day5::RangeCollection;
use nom::{
    bytes::complete::tag, character::complete::digit1, character::complete::multispace1,
    error::Error, multi::separated_list1, IResult,
};
use std::env;
use std::fs;
use indicatif::ProgressBar;

fn parse_numbers(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(multispace1, digit1)(input)
}

fn parse_seed_numbers(input: &str) -> IResult<&str, Vec<&str>> {
    let (numbers, _) = tag::<&str, &str, Error<_>>("seeds:")(input)?;
    let (numbers, _) = multispace1::<&str, Error<_>>(numbers)?;
    parse_numbers(numbers)
}

fn get_transition_ranges(contents: &String) -> (Vec<u64>,Vec<RangeCollection>){
    let mut lines = contents.lines();
    let seed_line = lines.next().unwrap();
    let (_, seed_numbers) = parse_seed_numbers(seed_line).unwrap();
    let seed_numbers: Vec<u64> = seed_numbers
        .iter()
        .map(|num| num.parse::<u64>().unwrap())
        .collect();
    let mut range_collection: Vec<RangeCollection> = Vec::new();
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
                let mut temp_range_vec:Vec<Range> = Vec::new();
                for map_line in &range_lines {
                    let (_, range_numbers) = parse_numbers(map_line).unwrap();
                    let range_numbers: Vec<u64> = range_numbers
                        .iter()
                        .map(|num| num.parse::<u64>().unwrap())
                        .collect();
                    let range = Range::new(range_numbers[0], range_numbers[1], range_numbers[2]);
                    temp_range_vec.push(range)
                }
                range_collection.push(RangeCollection::new(temp_range_vec));
            },
            _=>{}
        }
    }
    (seed_numbers,range_collection )
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
    let (seed_numbers, transitions )= get_transition_ranges(&contents);
    println!("File parsed");
    let mut min_value = u64::MAX;
    let bar = ProgressBar::new(seed_numbers.len() as u64);
    for i in (0..seed_numbers.len()).step_by(2){    
        for num in seed_numbers[i]..seed_numbers[i]+seed_numbers[i+1]{
            let mut seed = num;
            for transition in &transitions{
                match transition.translate_single(seed) {
                    None =>{},
                    Some(value) => {
                        seed = value;
                    }
                }
            }
            if seed < min_value {
                min_value = seed;
            }
        }
        bar.inc(2);
    }
    println!("{min_value}");
}
