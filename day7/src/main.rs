use nom::{
    AsChar,
    bytes::complete::tag, character::complete::digit1,
    IResult, sequence::separated_pair, bytes::complete::take_while
};
use std::env;
use std::fs;
use day7::Hand;

fn parse_hand(input: &str) -> IResult<&str, &str> {
    take_while(AsChar::is_alphanum)(input)
}

fn parse_hand_and_bet(input: &str) -> IResult<&str, (&str,&str)>{
    Ok(separated_pair(parse_hand, tag(" "), digit1)(input)?)
}

fn create_hand_vec(contents:&str) -> Vec<Hand>{
    let mut result: Vec<Hand> = Vec::new();
    for line in contents.lines(){
        let (_,(hand,bet)) = parse_hand_and_bet(line).unwrap();
        let bet = bet.parse::<u64>().unwrap();
        result.push(Hand::new(hand,bet));
    }
    result
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
    let mut hand_vec = create_hand_vec(&contents);
    hand_vec.sort();

    let mut result_part1 = 0;
    for (idx, item) in hand_vec.iter().enumerate() {
        let rank: u64 = (idx + 1) as u64;
        result_part1 += rank * item.bet;
    }
    println!("{:?}",result_part1);
}
