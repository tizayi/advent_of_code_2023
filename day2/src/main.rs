use std::collections::HashMap;
use std::env;
use std::fs;

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
    let bag: HashMap<&str, i32> =
    [("blue", 14),
     ("red", 12),
     ("green", 13)]
    .iter().collect();

    contents.lines().
}

fn process_game(line: &str, bag: &HashMap<&str, i32>) -> Option<u32> {
    // put fun code here
    let split  
}