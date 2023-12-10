use std::ops::Deref;

use itertools::Itertools;

use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

// #[derive(Debug)]
// struct Hand {
//     hand: HandType,
//     card_scores: Vec<u32>,
// }
fn score_hand(hand: &str) -> (HandType, (u32, u32, u32, u32, u32)) {
    use HandType::*;

    let counts = hand.chars().counts();
    let values = counts.values().sorted().join("");
    let hand_type = match values.deref() {
        "5" => FiveOfAKind,
        "14" => FourOfAKind,
        "23" => FullHouse,
        "113" => ThreeOfAKind,
        "122" => TwoPair,
        "1112" => OnePair,
        "11111" => HighCard,
        value => panic!("should never happen. Encountered `{}`", value),
    };
    let card_scores = hand
        .chars()
        .map(|card| match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            value => value.to_digit(10).unwrap(),
        })
        .collect_tuple()
        .unwrap();
    (hand_type, card_scores)
}

pub fn main() {
    let input = include_str!("../../data/data.txt");

    let hands = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(" ").unwrap();
            (hand, bid.parse::<u32>().unwrap(), score_hand(hand))
        })
        .sorted_by_key(|x| (x.2 .0 as u8, x.2 .1))
        .enumerate()
        .map(|(index, (_hand, bid, _))| (index as u32 + 1) * bid)
        .sum::<u32>();
    dbg!(hands.to_string());
}

#[derive(Error, Diagnostic, Debug)]
pub enum AocError {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    #[diagnostic(code(aoc::parse_int_error))]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error(transparent)]
    CardFromCharError(#[from] CardFromCharError),

    #[error("parse error: splitting strings")]
    #[diagnostic(code(aoc::parse::split))]
    SplitError {
        // The Source that we're gonna be printing snippets
        // out of. This can be a String if you
        // don't have or care about file names.
        #[source_code]
        src: NamedSource,
        // Snippets and highlights can be included in the
        // diagnostic!
        #[label("This bit here")]
        bad_bit: SourceSpan,
    },
}

#[derive(Error, Diagnostic, Debug)]
pub enum CardFromCharError {
    #[error("Not a valid card: `{0}`")]
    #[diagnostic(code(aoc::card::invalid_character))]
    InvalidCharacter(char),
}

#[derive(Error, Diagnostic, Debug)]
pub enum ScoreError {
    #[error("Not a valid card: `{0}`")]
    #[diagnostic(code(aoc::card::invalid_character))]
    InvalidCharacter(char),
}
