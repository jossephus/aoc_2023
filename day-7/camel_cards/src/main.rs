use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::{take, take_while1, take_while_m_n},
    combinator::{map, map_res},
    AsChar, IResult,
};

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq)]
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
    2, 1, 1
}

impl TryFrom<Vec<Card>> for HandKind {
    type Error = anyhow::Error;

    fn try_from(value: Vec<Card>) -> std::result::Result<Self, Self::Error> {
        // five_kind
        match value.iter().unique().count() {
            1 => Ok(HandKind::FIVE_KIND),
            5 => Ok(HandKind::HIGH),
            3 => Ok(HandKind::TWO_PAIR),
            _ => Ok(Self::NONE),
        }

        // high
    }
}

#[derive(Debug, PartialEq)]
struct Hand {
    cards: Vec<Card>,
    kind: HandKind,
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let cards = value
            .bytes()
            .filter_map(|x| x.as_char().try_into().ok())
            .collect::<Vec<Card>>();

        //let mut a = HashMap::new();
        //for (key, value) in cards.iter().group_by(|x| x) {}

        Hand {
            cards,
            kind: HandKind::FIVE_KIND,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.kind < other.kind {
            return Some(Ordering::Less);
        } else if self.kind > other.kind {
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

fn main() {
    let a = "hello";
    a.chars().next();
    println!("Hello, world!");
}

#[test]
fn test_parse_card() {
    assert_eq!(parse_card("A"), Ok(("", Card::A)));
}

#[test]
fn test_parse_hand() {
    //assert_eq!(
    //parse_hand("AAA22"),
    //Ok((
    //"",
    //Hand {
    //cards: vec![Card::A, Card::A, Card::A, Card::H2, Card::H3],
    //kind: HandKind::FULL_HOUSE,
    //}
    //))
    //)
}

#[test]
fn test() {
    let cards = vec![Card::A, Card::A, Card::A, Card::A, Card::A];
    let a: HandKind = cards.try_into().unwrap();
    assert_eq!(a, HandKind::FIVE_KIND,);
}
