use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::take_while_m_n;
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

fn parse_symbol(input: &str) -> IResult<&str, &str> {
    take_while_m_n(1, 1, |c: char| !c.is_digit(10) && c != '.')(input)
}

fn parse_line(input: &str) -> IResult<&str, (Vec<&str>, &str)> {
    many_till(alt((parse_dots, digit1, parse_symbol)), eof)(input)
}

fn expand_line_vec(line_vec: Vec<&str>) -> Vec<&str> {
    let mut new_line_vec: Vec<&str> = Vec::new();
    for item in line_vec {
        for _ in 0..item.len() {
            new_line_vec.push(item)
        }
    }
    new_line_vec
}

fn check_line_for_symbol(line_vec: &Vec<&str>, index_vec: &Vec<usize>) -> bool {
    for item_idx in index_vec {
        let item_str = line_vec[*item_idx];
        if item_str.chars().all(|c| !c.is_digit(10) && c != '.') {
            return true;
        }
    }
    false
}

fn check_line_for_num(gear_vec: &mut Vec<u32>, line_vec: &Vec<&str>, index_vec: &Vec<usize>) {
    let mut last_item: &str = "start";
    for item_idx in index_vec {
        let item_str = line_vec[*item_idx];
        match item_str.parse::<u32>() {
            Ok(num) => {
                if item_str != last_item {
                    gear_vec.push(num);
                }
            }
            _ => {}
        }
        last_item = item_str
    }
}

fn process_line(line: &str, last: Option<&str>, next: Option<&str>) -> (u32, u32) {
    let current = parse_line(line);
    let (_, (current_vec, _)) = current.unwrap();
    let last_vec: Option<Vec<&str>> = match last {
        Some(value) => {
            let (_, (last_vec, _)) = parse_line(value).unwrap();
            Some(expand_line_vec(last_vec))
        }
        None => None,
    };

    let next_vec: Option<Vec<&str>> = match next {
        Some(value) => {
            let (_, (next_vec, _)) = parse_line(value).unwrap();
            Some(expand_line_vec(next_vec))
        }
        None => None,
    };

    let mut part1_total = 0;

    let mut part2_total = 0;
    let mut counter: usize = 0;
    for (idx, item) in current_vec.iter().enumerate() {
        if *item == "*" {
            let mut need_check: Vec<usize> = Vec::new();
            let mut gear_total: Vec<u32> = Vec::new();

            if counter > 0 {
                need_check.push(counter - 1)
            }
            need_check.push(counter);

            if idx <= current_vec.len() {
                need_check.push(counter + item.len())
            }

            if idx > 0 {
                match current_vec[idx - 1].parse::<u32>() {
                    Ok(num) => gear_total.push(num),
                    _ => {}
                };
            }

            if idx < current_vec.len() - 1 {
                match current_vec[idx + 1].parse::<u32>() {
                    Ok(num) => {
                        gear_total.push(num);
                    }
                    _ => {}
                };
            }

            match &last_vec {
                Some(last) => check_line_for_num(&mut gear_total, &last, &need_check),
                _ => {}
            };

            match &next_vec {
                Some(value) => check_line_for_num(&mut gear_total, &value, &need_check),
                _ => {}
            };
            println!("{:?}", gear_total);
            if gear_total.len() == 2 {
                let gear_ratio = gear_total[0] * gear_total[1];
                part2_total += gear_ratio
            }
        }

        match item.parse::<u32>() {
            Ok(num) => {
                let mut need_check: Vec<usize> = Vec::new();

                if counter != 0 {
                    need_check.push(counter - 1)
                }

                if idx != current_vec.len() - 1 {
                    need_check.push(counter + item.len())
                }

                for i in counter..counter + item.len() {
                    need_check.push(i)
                }

                let symbol_in_last = match &last_vec {
                    Some(last) => check_line_for_symbol(&last, &need_check),
                    None => false,
                };

                let symbol_in_next = match &next_vec {
                    Some(value) => check_line_for_symbol(&value, &need_check),
                    None => false,
                };

                let mut symbol_next_to_number = false;

                if idx > 0 {
                    match current_vec[idx - 1].chars().last() {
                        Some(value) => {
                            if !value.is_digit(10) && value != '.' {
                                symbol_next_to_number = true
                            }
                        }
                        _ => {}
                    };
                }

                if idx < current_vec.len() - 1 {
                    match current_vec[idx + 1].chars().last() {
                        Some(value) => {
                            if !value.is_digit(10) && value != '.' {
                                symbol_next_to_number = true
                            }
                        }
                        _ => {}
                    };
                }

                if symbol_in_last || symbol_in_next || symbol_next_to_number {
                    part1_total += num
                }
            }
            Err(_) => {}
        };

        counter += item.len()
    }

    (part1_total, part2_total)
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
    let line_vec: Vec<&str> = contents.lines().collect();
    let mut result_part_1: u32 = 0;
    let mut result_part_2: u32 = 0;
    let mut last = None;
    for (idx, string_line) in line_vec.iter().enumerate() {
        let next_line;
        if idx == line_vec.len() - 1 {
            next_line = None
        } else {
            next_line = Some(line_vec[idx + 1])
        }

        let (num1, num2) = process_line(string_line, last, next_line);
        result_part_1 += num1;
        result_part_2 += num2;
        last = Some(string_line)
    }
    println!("part1: {:?} part2: {:?}", result_part_1, result_part_2);
}
