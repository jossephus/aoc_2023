use std::{collections::HashMap, fs};

fn main() {
    /*
     *
     *
     * */
    //let strings = vec![
    //"one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    //];

    //let digits = 1..9;

    //let mut strings_to_digits = strings.iter().zip(digits);

    //let mut is_lettered_digit = |i: &str| strings_to_digits.find(|(&x, y)| x == i);

    //if let Some(x) = is_lettered_digit("four") {
    //println!("{:?}", x.1);
    //}

    let data = fs::read_to_string("data/data.txt").unwrap();

    let splitted = data.split("\n").collect::<Vec<_>>();

    let mut total = 0;

    for i in splitted {
        let mut sum: Vec<char> = vec![];

        let numbers: Vec<char> = i.chars().filter(|x| x.is_numeric()).collect();
        println!("{:?}", numbers);

        if let Some(&first) = numbers.first() {
            sum.push(first);
        }

        if let Some(&last) = numbers.last() {
            sum.push(last);
        }

        total += sum.iter().collect::<String>().parse::<i32>().unwrap_or(0);
    }
    println!("Total: {:?}", total);
}
