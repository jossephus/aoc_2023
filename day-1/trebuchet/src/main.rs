use std::{collections::HashMap, fs, ops::Index};

fn main() {
    let data = fs::read_to_string("data/data.txt").unwrap();

    let numbers_to_strings = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    let splitted = data.trim().split("\n").collect::<Vec<_>>();

    let mut total = 0;

    for i in splitted {
        let mut sum: Vec<char> = vec![];

        let mut matches: Vec<(usize, char)> = vec![];

        for number in numbers_to_strings.keys() {
            if let Some(index) = i.find(number) {
                matches.push((index, numbers_to_strings.get(number).unwrap().to_owned()));
            }
        }

        let numbers: Vec<(usize, char)> = i
            .chars()
            .enumerate()
            .filter(|(index, x)| x.is_numeric())
            .map(|(x, c)| (x, c))
            .collect();

        matches.extend(numbers);

        matches.sort();

        if let Some(&first) = matches.first() {
            sum.push(first.1);
        }

        if let Some(&last) = matches.last() {
            sum.push(last.1);
        }

        total += sum.iter().collect::<String>().parse::<i32>().unwrap_or(0);
    }
    println!("Total: {:?}", total);
}
