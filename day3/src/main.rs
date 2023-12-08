use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::is_not;
use nom::bytes::complete::take_while;
use nom::character::complete::digit1;
use nom::combinator::eof;
use nom::error::Error;
use nom::multi::many_till;
use nom::IResult;
use std::env;
use std::fs;

fn parse_dots(input: &str) -> IResult<&str, &str> {
    is_a::<&str, &str, Error<_>>(".")(input)
}

fn is_not_digit(c: char) -> bool {
    !c.is_digit(10)
}

fn parse_symbol(input: &str) -> IResult<&str, &str> {
    alt((is_not("."), take_while(is_not_digit)))(input)
}

fn parse_line(input: &str) -> IResult<&str, (Vec<&str>, &str)> {
    many_till(alt((parse_dots, digit1, parse_symbol)), eof)(input)
}

fn check_line(line_vec: &Vec<&str>, index_vec: &Vec<usize>) -> bool {
    let mut counter = 0;
    let edge_list: Vec<usize> = line_vec
        .iter()
        .scan(0, |acc, item| {
            *acc = *acc + item.len();
            Some(*acc)
        })
        .collect();

    for (item_idx, edge) in edge_list.iter().enumerate() {
        for index in index_vec {
            if edge_list[item_idx - 1] < *index && index < edge {
                let new_string = String::from(line_vec[item_idx]);
                let char_at_index = new_string.chars().nth(edge - 1).unwrap();
                if !char_at_index.is_digit(10) && char_at_index != '.' {
                    return true;
                }
            }
        }
    }
    false
}

fn process_line(line: &str, last: Option<&str>, next: Option<&str>) -> u32 {
    let current = parse_line(line);
    let (_, (current_vec, _)) = current.unwrap();

    let last_vec: Option<Vec<&str>> = match last {
        Some(value) => {
            let (_, (last_vec, _)) = parse_line(value).unwrap();
            Some(last_vec)
        }
        None => None,
    };

    let next_vec: Option<Vec<&str>> = match next {
        Some(value) => {
            let (_, (next_vec, _)) = parse_line(value).unwrap();
            Some(next_vec)
        }
        None => None,
    };
    println!("{:?} {:?} {:?}", current_vec, next_vec, last_vec);

    let mut line_total = 0;

    let mut counter: usize = 0;

    for (idx, item) in current_vec.iter().enumerate() {
        match item.parse::<u32>() {
            Ok(num) => {
                let mut need_check: Vec<usize> = Vec::new();
                for i in counter - 1..counter + 2 {
                    need_check.push(i)
                }
                let symbol_in_last = match &last_vec {
                    Some(last) => check_line(&last, &need_check),
                    None => false,
                };

                let symbol_in_next = match &next_vec {
                    Some(value) => check_line(&value, &need_check),
                    None => false,
                };

                let mut symbol_next_to_number = false;

                match current_vec[idx - 1].chars().last() {
                    Some(value) => {
                        if !value.is_digit(10) && value != '.' {
                            symbol_next_to_number = true
                        }
                    }
                    _ => {}
                };

                match current_vec[idx + 1].chars().last() {
                    Some(value) => {
                        if !value.is_digit(10) && value != '.' {
                            symbol_next_to_number = true
                        }
                    }
                    _ => {}
                };

                if symbol_in_last || symbol_in_next || symbol_next_to_number {
                    line_total += num
                }
            }
            Err(_) => {}
        };

        counter += item.len()
    }

    println!("{:?}", line_total);
    line_total
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
    let contents = fs::read_to_string(file_path)
        .expect("File does not exist")

    for (idx, string_line) in contents.lines().enumerate() {
        let last = 
        process_line(string_line, last, next)
    }

    let _result2 = process_line("...*......", Some("467..114.."), Some("..35..633."));
}
