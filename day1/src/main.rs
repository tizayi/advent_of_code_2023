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
    let mut lowest: (&str, usize, usize) = ("fake",0,1000000);
    let mut highest: (&str, usize, usize) = ("fake",0, 0);
    for num in 0..digits.len() {
        for item in line.match_indices(digits[num]){
            if item.0 < lowest.2 {
                lowest = (digits[num], num+ 1, item.0 );
            }
            if item.0 > highest.2 {
                highest = (digits[num], num + 1, item.0 );
            }
        }
    }
    let mut new_line: String = String::from(line);
    let mut added = false;

    if lowest.0 != "fake"{
        new_line.insert(lowest.2 , char::from_digit(lowest.1 as u32,10).unwrap());
        added = true;
    }
    
    if lowest.2 != highest.2 && highest.0 != "fake" {
        if added {
            new_line.insert(highest.2 + 1, char::from_digit(highest.1 as u32, 10).unwrap());
        } else {
            new_line.insert(highest.2, char::from_digit(highest.1 as u32, 10).unwrap());}
    }

    get_first_and_last_numeric(&new_line)
}