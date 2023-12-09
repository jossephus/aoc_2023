use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::{tag, take, take_while1, take_while_m_n},
    character::is_digit,
    combinator::{all_consuming, map, map_res},
    sequence::{preceded, tuple},
    AsChar, Finish, IResult,
};

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord, Clone, Copy)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    H9,
    H8,
    H7,
    H6,
    H5,
    H4,
    H3,
    H2,
}

impl TryFrom<char> for Card {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            'A' => Ok(Card::A),
            'Q' => Ok(Card::Q),
            'J' => Ok(Card::J),
            'K' => Ok(Card::K),
            'T' => Ok(Card::T),
            '9' => Ok(Card::H9),
            '8' => Ok(Card::H8),
            '7' => Ok(Card::H7),
            '6' => Ok(Card::H6),
            '5' => Ok(Card::H5),
            '4' => Ok(Card::H4),
            '3' => Ok(Card::H3),
            '2' => Ok(Card::H2),
            _ => Err(anyhow!("We cant parse char")),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Eq, Ord)]
enum HandKind {
    FIVE_KIND,     // 5 - same
    FOUR_KIND,     // 4 - same, 1 - different
    FULL_HOUSE,    // 3 - same, 2 - same
    THREE_OF_KIND, // 3- same, 2 - different
    TWO_PAIR,      // 2 - same, 2 - same, 1 - different
    ONE_PAIR,      // 2 - same, 3 different
    HIGH,          // 5 - different
    NONE,
}

impl TryFrom<Vec<Card>> for HandKind {
    type Error = anyhow::Error;

    fn try_from(value: Vec<Card>) -> std::result::Result<Self, Self::Error> {
        // five_kind
        match value.iter().unique().collect::<Vec<_>>()[..] {
            [_] => Ok(HandKind::FIVE_KIND),
            [x, y] => match (value.iter().counts()[x], value.iter().counts()[y]) {
                (4, 1) | (1, 4) => Ok(HandKind::FOUR_KIND),
                (3, 2) | (2, 3) => Ok(HandKind::FULL_HOUSE),
                (a, b) => {
                    panic!("No we dont handle that {} {}", a, b)
                }
            },
            [x, y, z] => match (
                value.iter().counts()[x],
                value.iter().counts()[y],
                value.iter().counts()[z],
            ) {
                (3, 1, 1) | (1, 1, 3) | (1, 3, 1) => Ok(HandKind::THREE_OF_KIND),
                (2, 1, 2) | (2, 2, 1) | (1, 2, 2) => Ok(HandKind::TWO_PAIR),
                (a, b, c) => {
                    panic!("No we dont handle that {} {} {}", a, b, c)
                }
            },
            [_, _, _, _] => Ok(HandKind::ONE_PAIR),
            [_, _, _, _, _] => Ok(HandKind::HIGH),
            _ => {
                panic!("We dont handle this");
            }
        }
        // high
    }
}

#[derive(Debug, PartialEq)]
struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn kind(&self) -> HandKind {
        self.cards.to_owned().try_into().unwrap()
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let cards = value
            .bytes()
            .filter_map(|x| x.as_char().try_into().ok())
            .collect::<Vec<Card>>();

        Hand { cards }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let (self_kind, other_kind) = (self.kind(), other.kind());

        if self_kind < other_kind {
            return Some(Ordering::Less);
        } else if self_kind > other_kind {
            return Some(Ordering::Greater);
        }

        let mut i = 0;

        while self.cards[i] == other.cards[i] {
            i = i + 1;
        }

        return match self.cards[i].partial_cmp(&other.cards[i]) {
            Some(ordering) => Some(ordering),
            None => None,
        };
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Game {
    hand: Hand,
    bid: i32,
}

fn is_valid_hand(c: char) -> bool {
    "AQKJT98765432".contains(c)
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    map_res(take_while_m_n(1, 1, is_valid_hand), |x: &str| {
        x.chars().next().unwrap().try_into()
    })(input)
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    map_res(take_while1(is_valid_hand), |value: &str| value.try_into())(input)
}

fn parse_bid(input: &str) -> IResult<&str, i32> {
    map_res(take_while1(|u| is_digit(u as u8)), |a: &str| {
        a.parse::<i32>()
    })(input)
}

fn parse_line(input: &str) -> IResult<&str, Game> {
    map(tuple((parse_hand, tag(" "), parse_bid)), |(x, y, z)| Game {
        hand: x,
        bid: z,
    })(input)
}

fn main() {
    let data = include_str!("../../data/data.txt").lines();

    let mut games = data
        .map_while(|line| {
            all_consuming(parse_line)(line)
                .finish()
                .ok()
                .map(|(_, line)| line)
        })
        .collect_vec();

    let a = games
        .iter()
        .sorted_by(|a, b| a.hand.partial_cmp(&b.hand).unwrap())
        .rev()
        .enumerate()
        .map(|(index, game)| (index + 1) * game.bid as usize)
        .sum::<usize>();
    dbg!(a);
}

#[test]
fn test_parse_card() {
    assert_eq!(parse_card("A"), Ok(("", Card::A)));
}

#[test]
fn test_parse_hand() {
    assert_eq!(
        parse_hand("AAA22"),
        Ok((
            "",
            Hand {
                cards: vec![Card::A, Card::A, Card::A, Card::H2, Card::H2],
            }
        ))
    );
}

#[test]
fn test() {
    let cards = vec![Card::A, Card::A, Card::A, Card::A, Card::A];
    let a: HandKind = cards.try_into().unwrap();
    assert_eq!(a, HandKind::FIVE_KIND);

    let cards = vec![Card::A, Card::A, Card::A, Card::A, Card::Q];
    let a: HandKind = cards.try_into().unwrap();
    assert_eq!(a, HandKind::FOUR_KIND);

    let cards = vec![Card::A, Card::A, Card::A, Card::Q, Card::Q];
    let a: HandKind = cards.try_into().unwrap();
    assert_eq!(a, HandKind::FULL_HOUSE);

    let cards = vec![Card::A, Card::A, Card::Q, Card::A, Card::Q];
    let a: HandKind = cards.try_into().unwrap();
    assert_eq!(a, HandKind::FULL_HOUSE);

    let cards = vec![Card::A, Card::A, Card::A, Card::T, Card::Q];
    let a: HandKind = cards.try_into().unwrap();
    assert_eq!(a, HandKind::THREE_OF_KIND);

    let cards = vec![Card::T, Card::A, Card::A, Card::A, Card::Q];
    let a: HandKind = cards.try_into().unwrap();
    assert_eq!(a, HandKind::THREE_OF_KIND);
}
