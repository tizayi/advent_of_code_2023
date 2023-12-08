use std::env;
use std::fs;
use nom::{
    IResult
};

struct PartNumber<'a>{
    number: &'a str,
    last: Option<&'a str>,
    next: Option<&'a str>
}

fn processs_line(line: &str, last: Option<&str>, next: Option<&str> ) -> u32 {

    let curren_line = String::from(line);
    println!("{:?}", owned_line);
    
    
    
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
    processs_line("467..114..");

}
