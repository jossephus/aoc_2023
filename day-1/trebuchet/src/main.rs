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

    let total = data
        .trim()
        .split("\n")
        .into_iter()
        .enumerate()
        .map(|(index, mut i)| {
            let mut sum: Vec<char> = vec![];

            let mut matches: Vec<(usize, char)> = vec![];

            for number in numbers_to_strings.keys() {
                let mut j = i.to_string();
                while let Some(index) = j.find(number) {
                    matches.push((
                        j.find(number).unwrap(),
                        numbers_to_strings.get(number).unwrap().to_owned(),
                    ));
                    j.replace_range(
                        index..index + number.len(),
                        " ".repeat(number.len()).as_str(),
                    );
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
            sum.iter().collect::<String>().parse::<i32>().unwrap_or(0)
        })
        .fold(0, |total, x| total + x);

    println!("Total: {:?}", total);
}
