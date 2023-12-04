use std::collections::HashMap;
use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &str = match args.len() {
        2 => {
            println!("Finding calibration in {}",args[1]);
            &args[1]
        },
        _ => panic!("only one argument is needed")
    };
    let contents = fs::read_to_string(file_path).expect("File does not exist");
    let bag: HashMap<&str, u32> =
    [("blue", 14),
     ("red", 12),
     ("green", 13)]
    .iter().cloned().collect();

    
    process_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", &bag, &game_regex);
}

fn process_game(line: &str, bag: &HashMap<&str, i32>) -> Option<u32> {
    let blue_regex = Regex::new(r"\b\d+ blue\b").expect("regex is wrong");
    let green_regex = Regex::new(r"\b\d+ green\b").expect("regex is wrong");
    let red_regex = Regex::new(r"\b\d+ red\b").expect("regex is wrong");
    let game_regex = Regex::new(r"\b\d+\b").expect("regex is wrong");

    let blue_possible = true;
    let red_possible = true;
    let green_possible = true;


    let mut split = line.split(":");
    let caps = game_regex.captures(split.next().unwrap()).unwrap();
    let game_number = &caps[0];

    let hand_split = split.next().unwrap().split(";").unwrap();
    for hand in hand_split {
        let Some(caps) = blue_regex.captures(hand) else {
        continue
        };
        if &caps[0].parse().unwrap() as u32 > bag["blue"].try_into().unwrap(){
            blue_possible = false
        };
        let Some(caps) = green_regex.captures(hand) else {
            continue
        };
        if &caps[0] as u32 > bag["green"].try_into().unwrap(){
            green_possible = false
        };
        let Some(caps) = red_regex.captures(hand) else {
            continue
        };
        if &caps[0] as u32 > bag["red"].try_into().unwrap(){
            red_possible = false
        };
    }
    
    println!("{} {} {}", blue_possible, red_possible,green_possible)

    println!("{}",game_number);
    Some(4)
}