use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashSet, fs};

lazy_static! {
    static ref NUMBERS_REGEX: Regex = Regex::new(r"(\d+)").unwrap();
}

fn main() {
    let data = fs::read_to_string("data/data.txt").unwrap();

    let sum = data
        .lines()
        .map(|line| {
            let (card_name, remaining) = line.split_once(":").unwrap();

            let (winnig_cards, my_cards) = remaining.split_once("|").unwrap();

            (
                NUMBERS_REGEX
                    .find_iter(winnig_cards)
                    .map(|m| m.as_str())
                    .collect::<Vec<_>>(),
                NUMBERS_REGEX
                    .find_iter(my_cards)
                    .map(|m| m.as_str())
                    .collect::<Vec<_>>(),
            )
        })
        .map(|(mut winnig_cards, my_cards)| {
            winnig_cards.extend(my_cards);
            let init_length = winnig_cards.len();
            let final_length = winnig_cards.into_iter().collect::<HashSet<_>>().len();

            if init_length - final_length == 0 {
                0
            } else {
                2_u32.pow((init_length - final_length - 1) as u32)
            }
        })
        .sum::<u32>();
    println!("{:?}", sum);
}
