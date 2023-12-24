use nom::{
    AsChar,
    bytes::complete::tag, character::complete::digit1,
    IResult, sequence::separated_pair, bytes::complete::take_while
};
use std::env;
use std::fs;
use day7::compare_cards;

fn parse_hand(input: &str) -> IResult<&str, &str> {
    take_while(AsChar::is_alphanum)(input)
}

fn parse_hand_and_bet(input: &str) -> IResult<&str, (&str,&str)>{
    Ok(separated_pair(parse_hand, tag(" "), digit1)(input)?)
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




    let input = contents.lines().next().unwrap();
    println!("{:?}",&input);
    let result = parse_hand_and_bet(input);
    let stuff = compare_cards('6','7');
    println!("{:?}", stuff);
    println!("Hello, world!");
}
