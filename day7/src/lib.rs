use std::collections::HashMap;
use std::cmp;
#[derive(Copy, Clone, Debug)]
pub enum HandType {
    FiveKind = 6,
    FourKind = 5,
    FullHouse = 4,
    ThreeKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug)]
pub struct Hand{
    pub bet: u64,
    hand: String,
    hand_type: HandType
}

impl Hand{
    pub fn new(input_hand: &str, bet:u64 ) -> Hand{
        let hand_string = String::from(input_hand);
        let hand_type = get_hand_type(input_hand);
        Hand{
            bet,
            hand: hand_string,
            hand_type
        }
    }
}

impl PartialOrd for Hand{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>{
        if (self.hand_type as u8) > (other.hand_type as u8) {
            return Some(cmp::Ordering::Greater);
        } else if (self.hand_type as u8) < (other.hand_type as u8) {
            return Some(cmp::Ordering::Less);
        }
        for (char1, char2) in self.hand.chars().zip(other.hand.chars()){
            match compare_cards(char1,char2){
                None => {},
                Some(result) => {
                    if result {
                        return Some(cmp::Ordering::Greater);
                    }
                    return Some(cmp::Ordering::Less);
                }
            }
        }
        return Some(cmp::Ordering::Equal)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            if (self.hand_type as u8) > (other.hand_type as u8) {
                return cmp::Ordering::Greater;
            } else if (self.hand_type as u8) < (other.hand_type as u8) {
                return cmp::Ordering::Less;
            }
            for (char1, char2) in self.hand.chars().zip(other.hand.chars()){
                match compare_cards(char1,char2){
                    Some(result) => {
                        if result {
                            return cmp::Ordering::Greater;
                        }
                        return cmp::Ordering::Less;
                    },
                    _ => {}
                }
            }
            return cmp::Ordering::Equal;
        }
}

impl Eq for Hand {}


impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}


pub fn get_hand_type(input_vec: &str) -> HandType{
    let mut num_map: HashMap<char,u64> = HashMap::new();
    for item in input_vec.chars(){
        num_map.entry(item).and_modify(|counter| *counter += 1).or_insert(1);
    }
    let mut most_repeats: Vec<u64> = num_map.values().cloned().collect();
    most_repeats.sort();
    most_repeats.reverse();
    let most = most_repeats[0];

    println!("{:?}",jocker_bonus);
    match most {
        5 => {
            return HandType::FiveKind;
        }
        4 => {
            return HandType::FourKind;
        }
        3 => {
            let second_most = most_repeats[1];
            match second_most{
                2 => {return HandType::FullHouse},
                1 => {return HandType::ThreeKind},
                _ => {panic!()}
            }
        }
        2=>{
            let second_most = most_repeats[1];
            match second_most {
                2 => {return HandType::TwoPair},
                1 => {return HandType::OnePair},
                _ => {panic!()}
            }
        }
        1=> {
            return HandType::HighCard;
        }
        _ => {panic!()}
    }
}


pub fn face_card(input: char) -> u32{
    match input {
    'A'=> {14},
    'K'=> {13},
    'Q'=> {12},
    'J' => {1},
    'T'=> {10},
    _ => panic!()
    }
}

pub fn compare_cards(card1: char, card2: char)->Option<bool>{
    let card1 = match card1.to_digit(10){
        None =>{
            face_card(card1)
        },
        Some(value) => {
            value
        }
    };
    let card2 = match card2.to_digit(10){
        None =>{
            face_card(card2)
        },
        Some(value) => {
            value
        }
    };
    if card1 > card2 {
        return Some(true);
    };
    if card1 == card2{
        return None;
    }
    Some(false)
}