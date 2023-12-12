mod input;
use std::collections::HashMap;

struct Hand {
    cards: String,
    bid: i32,
}

impl Hand {
    fn new() -> Self {
        Hand {cards: String::from(""), bid: 0 as i32}
    }
}

fn get_hands(input: String) -> Vec<Hand> {
    let mut result : Vec<Hand> = vec![];
    let tmp_vec : Vec<Vec<String>> = input
        .lines()
        .map(|s| s
             .to_string()
             .split_whitespace()
             .map(|i| i.to_string())
             .collect())
        .collect();
    for vec in tmp_vec {
        let mut hand = Hand::new();
        hand.cards = vec[0].clone();
        hand.bid = vec[1].parse::<i32>().unwrap();
        result.push(hand);
    }
    return result
}

fn get_value(hand: Hand) -> i32 {
    let mut map = HashMap::new();
    for c in hand.cards.chars() {
        if Option::is_some(&map.get(&c)) {
            map.insert(c, map.get(&c).unwrap()+1);
        }
    }

    return 0 as i32
}

fn main() {
    let input = input::get_test_input();
    //println!("{}", input);
    let hands = get_hands(input);


}
