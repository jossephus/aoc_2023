use itertools::Itertools;
use std::cmp::Ordering;

use anyhow::{anyhow, Result};
use nom::bytes::complete::{tag, take_while1};
use nom::{
    character::is_digit,
    combinator::{all_consuming, map, map_res},
    sequence::tuple,
    AsChar, Finish, IResult,
};

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord, Clone, Copy)]
enum Card {
    J = 3,
    H2,
    H3,
    H4,
    H5,
    H6,
    H7,
    H8,
    H9,
    T,
    Q,
    K,
    A,
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
    High = 1,    // 5 - different
    OnePair,     // 2 - same, 3 different
    TwoPair,     // 2 - same, 2 - same, 1 - different
    ThreeOfKind, // 3- same, 2 - different
    FullHouse,   // 3 - same, 2 - same
    FourKind,    // 4 - same, 1 - different
    FiveKind,    // 5 - same
}

impl TryFrom<Vec<Card>> for HandKind {
    type Error = anyhow::Error;

    fn try_from(value: Vec<Card>) -> std::result::Result<Self, Self::Error> {
        // five_kind
        let mut value = value;

        if let Some(_) = value.iter().find(|&x| x == &Card::J) {
            if value.iter().counts()[&Card::J] != 5 {
                let highest = value
                    .iter()
                    .filter(|&&x| x != Card::J)
                    .counts()
                    .into_iter()
                    .max_by(|(_, v), (_, p)| v.cmp(p))
                    .unwrap()
                    .0;

                value = value
                    .iter()
                    .map(|&x| if x == Card::J { *highest } else { x })
                    .collect_vec();
            }
        }

        match value.iter().unique().collect::<Vec<_>>()[..] {
            [_] => Ok(HandKind::FiveKind),
            [x, y] => match (value.iter().counts()[x], value.iter().counts()[y]) {
                (4, 1) | (1, 4) => Ok(HandKind::FourKind),
                (3, 2) | (2, 3) => Ok(HandKind::FullHouse),
                (a, b) => {
                    panic!("No we dont handle that {} {}", a, b)
                }
            },
            [x, y, z] => match (
                value.iter().counts()[x],
                value.iter().counts()[y],
                value.iter().counts()[z],
            ) {
                (3, 1, 1) | (1, 1, 3) | (1, 3, 1) => Ok(HandKind::ThreeOfKind),
                (2, 1, 2) | (2, 2, 1) | (1, 2, 2) => Ok(HandKind::TwoPair),
                (a, b, c) => {
                    panic!("No we dont handle that {} {} {}", a, b, c)
                }
            },
            [_, _, _, _] => Ok(HandKind::OnePair),
            [_, _, _, _, _] => Ok(HandKind::High),
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

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.hand.cards, self.bid)
    }
}

fn is_valid_hand(c: char) -> bool {
    "AQKJT98765432".contains(c)
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
    map(tuple((parse_hand, tag(" "), parse_bid)), |(x, _, z)| Game {
        hand: x,
        bid: z,
    })(input)
}

fn main() {
    let data = include_str!("../data/data.txt").lines();

    let games = data
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
        .enumerate()
        .inspect(|(_, game)| {
            println!("{}", game);
        })
        .map(|(index, game)| (index + 1) * game.bid as usize)
        .sum::<usize>();
    dbg!(a);
}
