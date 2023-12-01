use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &str = match args.len() {
        2 => {
            println!("Finding calibration in {}",args[1]);
            &args[1]
        },
        _ => panic!("only one argument is needed")
    };
    let contents = fs::read_to_string(file_path).unwrap();
    let calibrations: Vec<u32> = contents.lines().map(|line| get_first_and_last_digit(line)).collect();
    let calibration_sum: u32 = calibrations.iter().sum();
    println!("The calibration sum is: {}",calibration_sum);
}

fn get_first_and_last_numeric(line: &str) -> u32 {
    let nums: Vec<char> = line.chars().filter(|item| item.is_numeric()).collect();
    let first:u32 = nums.first().unwrap().to_digit(10).unwrap();
    let last:u32 = nums.last().unwrap().to_digit(10).unwrap();
    first*10 + last
}

fn get_first_and_last_digit(line: &str) -> u32 {
    let digits: Vec<&str> = vec!["one", "two", "three", "four", "five", "six", "seven", "eight","nine"];
    let mut lowest: Option<(usize, usize)> = None;
    let mut highest: Option<(usize, usize)> = None;

    for num in 0..digits.len() {
        for item in line.match_indices(digits[num]){
            match lowest {
                None => {lowest = Some(( num + 1, item.0 ));}
                Some(value) => {
                    if item.0 < value.1 {
                        lowest = Some(( num+ 1, item.0 ));
                    }
                }
            }
            match highest {
                None => highest = Some(( num + 1, item.0 )),
                Some(value) => {
                    if item.0 > value.1 {
                    highest = Some(( num + 1, item.0 ));
                }}
            }
        }
    }

    let mut new_line: String = String::from(line);
    let mut added = false;

    match lowest{ 
        Some(value) => {
            new_line.insert(value.1 , char::from_digit(value.0 as u32,10).unwrap());
            added = true;
        },
        None => {}
    }

    match highest {
        Some(value) => {
            if added {
                new_line.insert(value.1 + 1, char::from_digit(value.0 as u32, 10).unwrap());} 
            else {
            new_line.insert(value.1, char::from_digit(value.0 as u32, 10).unwrap());}
        },
        None => {}
    }

    get_first_and_last_numeric(&new_line)
}