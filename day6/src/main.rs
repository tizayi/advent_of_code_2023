use std::env;
use std::fs;
use day6::{parse_time_records,possible_record_breaks,parse_time_records_single };

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
    let (time_numbers,record_numbers) = parse_time_records(&contents);
    let mut result:u64 = 1;

    // part 1
    for i in 0..time_numbers.len(){
        result*=possible_record_breaks(time_numbers[i],record_numbers[i]);
    }
    println!("part1: {:?}",result);

    // part 2
    let part2_numbers = parse_time_records_single(&contents);
    let result2 = possible_record_breaks(part2_numbers.0,part2_numbers.1);
    println!("part2: {:?}",result2);
}
