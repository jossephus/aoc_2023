use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

lazy_static! {
    static ref NUMBERS_REGEX: Regex = Regex::new(r"(\d+)").unwrap();
}

#[derive(Debug)]
struct Card {
    id: usize,
    matching_numbers: usize,
}

fn main() {
    let data = fs::read_to_string("data/data.txt").unwrap();

    let mut id_to_matching_numbers: HashMap<_, _> = HashMap::new();

    let mut cards = data
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
        //.fold(vec![], |mut v, x| );
        .map(|(mut winnig_cards, my_cards)| {
            winnig_cards.extend(my_cards);
            let init_length = winnig_cards.len();
            let final_length = winnig_cards.into_iter().collect::<HashSet<_>>().len();

            (init_length - final_length) as u32
        })
        .enumerate()
        .map(|(index, matching_numbers)| {
            id_to_matching_numbers.insert(index + 1, matching_numbers as usize);
            Card {
                matching_numbers: matching_numbers as usize,
                id: index + 1,
            }
        })
        .collect::<Vec<_>>();

    for card in &cards {
        let mut j = card.id + 1;

        while j <= card.id + card.matching_numbers {
            let other_card = Card {
                id: j,
                matching_numbers: *id_to_matching_numbers.get(&j).unwrap(),
            };
            cards.push(other_card);
            j = j + 1;
        }
    }

    println!("{:?}", cards);
    //.fold(vec![], |mut acc, (index, value)| {
    //acc.push(Card {
    //id: index + 1,
    //from: index + 1,
    //});

    //let mut v = value;
    //while v != 0 {
    //acc.push(Card {
    //id: (v as usize + index + 1) as usize,
    //from: index + 1,
    //});
    //v = v - 1;
    //}

    ////let abc = acc.get(..).unwrap();

    ////abc.iter().filter(|&&x| x.id == index + 1).for_each(|x| {
    ////let mut v = value;
    ////while v != 0 {
    ////acc.push(Card {
    ////id: (v as usize + index + 1) as usize,
    ////from: index + 1,
    ////});
    ////v = v - 1;
    ////}
    ////});

    //acc
    //});

    //dbg!(abc.iter().filter(|x| x.id == 2).count());
    //println!("{:?}", abc);
}

/*
 * (1, 2), (1, 3), (1, 4), (1, 5)
 *
 * */
