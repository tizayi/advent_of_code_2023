use std::env;
use std::fs;
use nom::{
    IResult,
};

fn parse_line(&str) -> IResult<&str,Vec<&str>> {
    alt(digit1)
}

fn validate_part(part_number: &PartNumber, line_vec: &Vec<&str>) -> u32 {
    let prevous = line_vec[part_number.line - 1]
    let line = line_vec[part_number.line]
    let next = line_vec[part_number.line + 1]
    let 
}   

struct PartNumber {
    line: u32,
    idicies: Vec<u32>,
    value: u32,
    is_part: Option<bool>,
}


fn is_valid(line:&str, last_line: &str, next_line: &str) -> u32 {

    4
}

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
    println!("{:?}", contents);
}
