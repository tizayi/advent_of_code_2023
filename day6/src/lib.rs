use nom::{
    bytes::complete::tag, character::complete::digit1, character::complete::multispace1,
    error::Error, multi::separated_list1, IResult,
};

fn parse_numbers(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(multispace1, digit1)(input)
}

fn parse_line_numbers<'a>(input: &'a str, line_tag:&'a str) -> IResult<&'a str, Vec<&'a str>> {
    let (numbers, _) = tag::<&str, &str, Error<_>>(line_tag)(input)?;
    let (numbers, _) = multispace1::<&str, Error<_>>(numbers)?;
    parse_numbers(numbers)
}

pub fn parse_time_records(contents: &str) ->(Vec<u64>,Vec<u64>){
    let mut lines = contents.lines();
    let time_line = lines.next().unwrap();
    let (_,time_numbers) = parse_line_numbers(time_line,"Time:").unwrap();
    let time_numbers: Vec<u64> = time_numbers
    .iter()
    .map(|num| num.parse::<u64>().unwrap())
    .collect();
    let record = lines.next().unwrap();
    let (_,record_numbers) = parse_line_numbers(record, "Distance:").unwrap();
    let record_numbers: Vec<u64> = record_numbers
    .iter()
    .map(|num| num.parse::<u64>().unwrap())
    .collect();
    (time_numbers,record_numbers)
}

pub fn parse_time_records_single(contents: &str)->(u64,u64){
    let mut lines = contents.lines();
    let time_line = lines.next().unwrap();
    let (_,time_numbers) = parse_line_numbers(time_line,"Time:").unwrap();
    let mut time_numbers_string = String::new();
    for item in time_numbers{
        time_numbers_string.push_str(item);
    }

    let record = lines.next().unwrap();
    let (_,record_numbers) = parse_line_numbers(record, "Distance:").unwrap();
    let mut record_numbers_string = String::new();
    for item in record_numbers{
        record_numbers_string.push_str(item);
    }

    (time_numbers_string.parse::<u64>().unwrap(),record_numbers_string.parse::<u64>().unwrap())
}

fn distance_traveled(wait_time: u64,total_time: u64) -> u64{
    (total_time - wait_time)*wait_time
}

pub fn possible_record_breaks(time: u64, record: u64) -> u64{
    let mut result: u64 = 0;
    for num in 0..time {
        let distance = distance_traveled(num,time);
        if distance > record{
            result +=1;
        }
    };
    result
}