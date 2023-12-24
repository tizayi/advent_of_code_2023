use std::collections::HashMap;

enum HandType {
    FiveKind = 6,
    FourKind = 5,
    FullHouse = 4,
    ThreeKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

pub struct Hand{
    bet: u64,
    hand: String
}

fn face_card(input: char) -> u32{
    match input {
    'A'=> {14},
    'K'=> {13},
    'Q'=> {12},
    'J' => {11},
    'T'=> {10},
    _ => panic!()
    }
}

pub fn compare_cards(card1: char, card2: char)->bool{
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
        return true;
    };
    false
}