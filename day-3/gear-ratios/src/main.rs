use std::{
    collections::{HashMap, HashSet},
    fs,
    str::FromStr,
};

use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Number<'a> {
    content: &'a str,
    /*index*/ index: usize,
    /*start*/ start: usize,
    /*end*/ end: usize,
}

#[derive(Debug)]
struct Symbol<'a> {
    content: &'a str,
    index: usize,
    start: usize,
}

lazy_static! {
    static ref NUMBERS_REGEX: Regex = Regex::new(r"(\d+)").unwrap();
    static ref SYMBOLS_REGEX: Regex = Regex::new(r"[^\d\.\n]+").unwrap();
}

fn main() {
    let data = fs::read_to_string("data/data.txt").unwrap();

    let line_length = data.lines().last().unwrap().len();

    let all_numbers = NUMBERS_REGEX
        .find_iter(data.as_str())
        .map(|x| Number {
            content: x.as_str(),
            index: x.start() / data.lines().last().unwrap().len(),
            start: x.start(),
            end: x.end(),
        })
        .collect::<Vec<_>>();

    let all_symboles = SYMBOLS_REGEX
        .find_iter(data.as_str())
        .map(|x| x.as_str().chars().next().unwrap())
        .collect::<HashSet<_>>();

    //.map(|x| Symbol {
    //content: x.as_str(),
    //index: (x.start() / data.lines().last().unwrap().len()),
    //start: x.start(),
    //})
    //.collect::<Vec<_>>();

    let sum = all_numbers
        .iter()
        .map(|number| {
            let before = if number.start != 0 {
                data.chars().nth(number.start - 1).unwrap()
            } else {
                '.'
            };

            let after = if number.end != data.len() - 1 {
                data.chars().nth(number.end).unwrap()
            } else {
                '.'
            };

            let top_start = if number.index != 0 {
                data.chars().nth(number.start - (line_length + 1)).unwrap()
            } else {
                '.'
            };

            let bottom_end = if let Some(x) = data.chars().nth(number.end + line_length) {
                x
            } else {
                '.'
            };

            let top_end = if number.index != 0 {
                data.chars().nth(number.end - (line_length + 2)).unwrap()
            } else {
                '.'
            };

            let bottom_start = if let Some(x) = data.chars().nth(number.start + line_length + 1) {
                x
            } else {
                '.'
            };

            let diagonal_bottom_end =
                if let Some(x) = data.chars().nth(number.end + line_length + 1) {
                    x
                } else {
                    '.'
                };

            let diagonal_bottom_start =
                if let Some(x) = data.chars().nth(number.start + line_length) {
                    x
                } else {
                    '.'
                };

            let diagonal_top_end = if number.index != 0 {
                data.chars().nth(number.end - (line_length + 1)).unwrap()
            } else {
                '.'
            };

            let diagonal_top_start = if number.index != 0 {
                data.chars().nth(number.start - line_length - 2).unwrap()
            } else {
                '.'
            };

            let symbols = vec!['*', '-', '#', '&', '+', '$', '%', '=', '@', '/'];

            let valid = vec![
                diagonal_bottom_end,
                diagonal_bottom_start,
                diagonal_top_end,
                diagonal_top_start,
                before,
                after,
                top_start,
                bottom_end,
                top_end,
                bottom_start,
            ]
            .iter()
            .any(|x| all_symboles.contains(x));

            if valid {
                let value = number.content.parse::<i32>().unwrap().to_owned();
                println!("{}", number.content);
                value
            } else {
                0
            }
        })
        .reduce(|acc, e| acc + e);

    println!("{:?}", sum);

    //dbg!(all_numbers);
    //dbg!(all_symboles);

    //let sum: i32 = data
    //.lines()
    //.enumerate()
    //.map(|(index, line)| {
    //NUMBERS_REGEX.find_iter(line).for_each(|m| {
    ////
    //let start = m.start();
    //let end = m.end();

    //// before
    //let mut sides: Vec<char> = vec![];

    //if start != 0 {
    //if let Some(ch) = line.chars().nth(0) {
    //sides.push(ch);
    //}
    //}

    //if let Some(ch) = line.chars().nth(end + 1) {
    //sides.push(ch);
    //}

    //// bottom-start
    //if let Some(ch) = data.chars().nth((index + 1) * line.len() + start) {
    //sides.push(ch);
    //}

    //// top-start
    //if index != 0 {
    //if let Some(ch) = data.chars().nth((index - 1) * line.len() + start) {
    //sides.push(ch);
    //}
    //}

    //if start != 0 {
    //// diagonal-bottom-start
    //if let Some(ch) = data.chars().nth((index + 1) * line.len() + (start - 1)) {
    //sides.push(ch);
    //}
    //// diagonal-top-start
    //if index != 0 {
    //if let Some(ch) = data.chars().nth((index - 1) * line.len() + start) {
    //sides.push(ch);
    //}
    //}
    //}

    //// bottom-end
    //if let Some(ch) = data.chars().nth((index + 1) * line.len() + (end + 1)) {
    //sides.push(ch);
    //}

    //// top-end
    //if index != 0 {
    //if let Some(ch) = data.chars().nth((index - 1) * line.len() + (start + 1)) {
    //sides.push(ch);
    //}
    //}

    //dbg!(sides);
    ////dbg!(m.as_str());
    //});
    //index as i32
    //})
    //.sum();

    //print!("{:?}", sum);
    println!("{:?}", all_symboles);
}
