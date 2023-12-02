use std::{fs, str::FromStr};

use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum Color {
    Red(i32),
    Blue(i32),
    Green(i32),
}

impl FromStr for Color {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some(index) = s.find("green") {
            return Ok(Color::Green(
                s.split_at(index).0.trim().parse::<i32>().unwrap(),
            ));
        } else if let Some(index) = s.find("blue") {
            return Ok(Color::Blue(
                s.split_at(index).0.trim().parse::<i32>().unwrap(),
            ));
        } else if let Some(index) = s.find("red") {
            return Ok(Color::Red(
                s.split_at(index).0.trim().parse::<i32>().unwrap(),
            ));
        }
        return Ok(Color::Green(3));
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Set {
    green: Color,
    red: Color,
    blue: Color,
}

impl Default for Set {
    fn default() -> Self {
        Self {
            green: Color::Green(0),
            blue: Color::Blue(0),
            red: Color::Red(0),
        }
    }
}

impl FromStr for Set {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut set = Set::default();

        s.split(",")
            .map(|s| s.parse::<Color>().unwrap())
            .for_each(|x| match x {
                Color::Red(n) => set.red = Color::Red(n),
                Color::Blue(n) => set.blue = Color::Blue(n),
                Color::Green(n) => set.green = Color::Green(n),
            });

        Ok(set)
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Cube {
    id: i32,
    sets: Vec<Set>,
}

impl FromStr for Cube {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (first, rest) = s.split_once(":").unwrap();

        let sets = rest
            .split(";")
            .map(|x| x.parse::<Set>().unwrap())
            .collect::<Vec<Set>>();

        Ok({
            Cube {
                id: first.split_once(" ").unwrap().1.parse::<i32>().unwrap(),
                sets,
            }
        })
    }
}

fn main() {
    let data = fs::read_to_string("data/data.txt").unwrap();

    let a: i32 = data
        .lines()
        .map(|line| parse_program_part_2(line, 12, 14, 13))
        .fold(0, |total, x| total + x);
    print!("{:?}", a);

    //let a: i32 = data
    //.lines()
    //.map(|line| parse_program(line, 12, 14, 13))
    //.filter(|line| line.is_ok())
    //.fold(0, |total, x| total + x.unwrap());
}

fn parse_program_part_1(line: &str, r: i32, b: i32, g: i32) -> Result<i32> {
    let cube = line.parse::<Cube>().unwrap();

    let mut error_msg = "";

    cube.sets.into_iter().for_each(|x| {
        match x {
            Set { green, red, blue } => {
                if green > Color::Green(g) {
                    error_msg = "Somehting Green";
                } else if red > Color::Red(r) {
                    error_msg = "Somehting Red";
                } else if blue > Color::Blue(b) {
                    error_msg = "Somehting Blue";
                }
            }
        };
    });

    if !error_msg.is_empty() {
        return Err(anyhow!(error_msg));
    }

    Ok(cube.id)
}
fn parse_program_part_2(line: &str, r: i32, b: i32, g: i32) -> i32 {
    let cube = line.parse::<Cube>().unwrap();

    let mut error_msg = "";

    let max_red = cube
        .clone()
        .sets
        .into_iter()
        .max_by(|x, y| (&x.red).partial_cmp(&y.red).unwrap())
        .unwrap()
        .red;

    let max_blue = cube
        .clone()
        .sets
        .into_iter()
        .max_by(|x, y| (&x.blue).partial_cmp(&y.blue).unwrap())
        .unwrap()
        .blue;
    let max_green = cube
        .clone()
        .sets
        .into_iter()
        .max_by(|x, y| (&x.green).partial_cmp(&y.green).unwrap())
        .unwrap()
        .green;

    let mut product = 1;

    if let Color::Red(x) = max_red {
        product *= x;
    }
    if let Color::Blue(x) = max_blue {
        product *= x;
    }
    if let Color::Green(x) = max_green {
        product *= x;
    }

    product
}

#[cfg(test)]
mod test {
    use crate::*;
    //#[test]
    //fn parse_color() {
    //assert_eq!("3 green".parse::<Color>().unwrap(), Color::Green(3));
    //assert_eq!("2 blue".parse::<Color>().unwrap(), Color::Blue(2));
    //assert_eq!("2 red".parse::<Color>().unwrap(), Color::Red(2));
    //}

    //#[test]
    //fn parse_sets() {
    //assert_eq!(
    //"4 blue, 3 green, 2 red".parse::<Set>().unwrap(),
    //Set {
    //green: Color::Green(3),
    //red: Color::Red(2),
    //blue: Color::Blue(4),
    //}
    //);
    //assert_eq!(
    //"4 blue".parse::<Set>().unwrap(),
    //Set {
    //green: Color::Green(0),
    //red: Color::Red(0),
    //blue: Color::Blue(4),
    //}
    //);
    //}

    #[test]
    fn draw() {
        assert_eq!(
            "Game 1: 3 blue,  2 red; 3 green, 2 blue"
                .parse::<Cube>()
                .unwrap(),
            Cube {
                id: 1,
                sets: vec![
                    Set {
                        blue: Color::Blue(3),
                        red: Color::Red(2),
                        green: Color::Green(0)
                    },
                    Set {
                        blue: Color::Blue(2),
                        red: Color::Red(0),
                        green: Color::Green(3),
                    }
                ]
            }
        )
    }
}
