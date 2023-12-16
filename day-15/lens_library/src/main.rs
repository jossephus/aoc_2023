use std::collections::{BTreeMap, HashMap};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_while1},
    character::{is_alphabetic, is_alphanumeric, is_digit},
    combinator::{map, map_res, opt},
    IResult,
};

#[derive(Clone)]
struct Instruction {
    length: Option<usize>,
    label: String,
    operation: char,
}

impl Instruction {
    fn hash(&self) -> usize {
        let mut label_operation = self.label.clone();
        label_operation.push(self.operation);
        if let Some(value) = self.length {
            label_operation.push(char::from_digit(value as u32, 10).unwrap());
        }
        dbg!(&label_operation);

        label_operation
            .chars()
            .fold(0, |acc, x| ((acc + x as usize) * 17) % 256)
    }

    fn hash_label(&self) -> usize {
        self.label
            .chars()
            .fold(0, |acc, x| ((acc + x as usize) * 17) % 256)
    }
}

impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}
impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Instruction: {} {} {:?}",
            self.label, self.operation, self.length
        )
    }
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, label) = take_while1(|c| is_alphanumeric(c as u8))(input)?;
    let (input, operation): (&str, char) = map(alt((tag("="), tag("-"))), |s: &str| {
        s.chars().into_iter().last().unwrap()
    })(input)?;

    let (input, length) = opt(map_res(take_while1(|u| is_digit(u as u8)), |a: &str| {
        a.parse::<usize>()
    }))(input)?;

    Ok((
        input,
        Instruction {
            label: label.to_string(),
            operation,
            length,
        },
    ))
}

fn main() {
    let data = include_str!("../data/data.txt");
    //let data = "HASH";
    let mut entries: BTreeMap<usize, Vec<Instruction>> = BTreeMap::new();

    data.lines().filter(|x| !x.is_empty()).for_each(|line| {
        //let line = x.split_once(",").unwrap().1.chars();
        line.split(",").into_iter().for_each(|splitted| {
            let instruction = parse_instruction(splitted).unwrap().1;
            entries
                .entry(instruction.clone().hash_label())
                .and_modify(|vecs| {
                    if let Some(index) = vecs.into_iter().position(|x| *x == instruction) {
                        if instruction.operation == '=' {
                            vecs[index] = instruction.clone();
                        } else {
                            vecs.remove(index);
                        }
                    } else {
                        vecs.push(instruction.clone());
                    }
                })
                .or_insert(vec![instruction.clone()]);
        });
    });

    let mut sum = 0;
    for key in entries.keys() {
        entries
            .get(key)
            .unwrap()
            .into_iter()
            .enumerate()
            .for_each(|(index, ins)| {
                let val = (key + 1) * (index + 1) * (ins.length.unwrap_or(0));
                sum += val;
            });
    }
    println!("{:?}", sum);
}

fn part_1(input: &str) {
    let data = include_str!("../data/data.txt");
    //let data = "HASH";

    let sum = data
        .lines()
        .map(|line| {
            //let line = x.split_once(",").unwrap().1.chars();
            let hash = line.split(",").into_iter().fold(0, |acc, splitted| {
                let hash = splitted.chars().fold(0, |acc, x| {
                    println!("{} {}", x, x as usize);
                    ((acc + x as usize) * 17) % 256
                });
                acc + splitted
                    .chars()
                    .fold(0, |acc, x| ((acc + x as usize) * 17) % 256)
            });

            hash
        })
        .collect::<Vec<_>>();
    dbg!(sum);
}

#[test]
fn test_parse_box() {
    assert_eq!(
        parse_instruction("cp=2"),
        Ok((
            "",
            Instruction {
                label: "cp".to_string(),
                operation: '=',
                length: Some(2),
            }
        ))
    );

    assert_eq!(
        parse_instruction("cp-"),
        Ok((
            "",
            Instruction {
                label: "cp".to_string(),
                operation: '-',
                length: None,
            }
        ))
    )
}
