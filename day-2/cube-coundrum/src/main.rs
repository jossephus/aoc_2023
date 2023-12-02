use std::{fs, str::FromStr};

use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum Color {
    Red { amount: i32 },
    Blue { amount: i32 },
    Green { amount: i32 },
}

impl FromStr for Color {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some(index) = s.find("green") {
            return Ok(Color::Green {
                amount: s.split_at(index).0.trim().parse::<i32>().unwrap(),
            });
        } else if let Some(index) = s.find("blue") {
            return Ok(Color::Blue {
                amount: s.split_at(index).0.trim().parse::<i32>().unwrap(),
            });
        } else if let Some(index) = s.find("red") {
            return Ok(Color::Red {
                amount: s.split_at(index).0.trim().parse::<i32>().unwrap(),
            });
        }

        return Err(anyhow!("Can not parse this color"));
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
            green: Color::Green { amount: 0 },
            blue: Color::Blue { amount: 0 },
            red: Color::Red { amount: 0 },
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
                Color::Red { amount } => set.red = Color::Red { amount },
                Color::Blue { amount } => set.blue = Color::Blue { amount },
                Color::Green { amount } => set.green = Color::Green { amount },
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
                match green {
                    Color::Green { amount } => error_msg = "something wrong",
                    _ => {}
                };

                match red {
                    Color::Red { amount } => error_msg = "something wrong",
                    _ => {}
                };

                match blue {
                    Color::Blue { amount } => error_msg = "something wrong",
                    _ => {}
                };
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

    vec![max_red, max_blue, max_green]
        .iter()
        .map(|x| match x {
            Color::Red { amount } => amount,
            Color::Blue { amount } => amount,
            Color::Green { amount } => amount,
        })
        .fold(1, |acc, e| acc * e)
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn parse_color() {
        assert_eq!(
            "3 green".parse::<Color>().unwrap(),
            Color::Green { amount: 3 }
        );
        assert_eq!(
            "2 blue".parse::<Color>().unwrap(),
            Color::Blue { amount: 2 }
        );
        assert_eq!("2 red".parse::<Color>().unwrap(), Color::Red { amount: 2 });
    }

    #[test]
    fn parse_sets() {
        assert_eq!(
            "4 blue, 3 green, 2 red".parse::<Set>().unwrap(),
            Set {
                green: Color::Green { amount: 3 },
                red: Color::Red { amount: 2 },
                blue: Color::Blue { amount: 4 },
            }
        );
        assert_eq!(
            "4 blue".parse::<Set>().unwrap(),
            Set {
                green: Color::Green { amount: 0 },
                red: Color::Red { amount: 0 },
                blue: Color::Blue { amount: 4 },
            }
        );
    }

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
                        blue: Color::Blue { amount: 3 },
                        red: Color::Red { amount: 2 },
                        green: Color::Green { amount: 0 }
                    },
                    Set {
                        blue: Color::Blue { amount: 2 },
                        red: Color::Red { amount: 0 },
                        green: Color::Green { amount: 3 },
                    }
                ]
            }
        )
    }
}
