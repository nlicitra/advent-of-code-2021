#![allow(unused)]

use std::fmt::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_bits() -> Vec<Vec<u8>> {
    let input_path = "./input/day3.txt";
    read_lines(input_path)
        .expect(format!("Failed to open input file: {}", input_path).as_str())
        .into_iter()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn part1() -> i32 {
    println!("Part 1");
    let bit_list = get_bits();
    let mut counter: [[u32; 2]; 12] = [[0, 0]; 12];
    for bits in bit_list {
        for index in 0..12 {
            counter[index][bits[index] as usize] += 1;
        }
    }
    let gamma = counter
        .map(|digit| match digit[0] > digit[1] {
            true => "0",
            false => "1",
        })
        .join("");
    let gamma = i32::from_str_radix(&gamma, 2).unwrap();

    let epsilon = counter
        .map(|digit| match digit[0] <= digit[1] {
            true => "0",
            false => "1",
        })
        .join("");
    let epsilon = i32::from_str_radix(&epsilon, 2).unwrap();
    gamma * epsilon
}

fn bit_list_to_number(bit_list: Vec<u8>) -> i32 {
    let bit_seq_string: Vec<String> = bit_list.into_iter().map(|b| b.to_string()).collect();
    let bit_seq_string = bit_seq_string.join("");
    i32::from_str_radix(&bit_seq_string, 2).unwrap()
}

type ByteList = Vec<Vec<u8>>;
enum GeneratorRating {
    OXYGEN,
    CO2,
}

enum RecursionStatus<T> {
    DONE(T),
    CONTINUE(Vec<T>),
}
fn filter_by_bit_index(
    list: &ByteList,
    bit_index: usize,
    generator_rating: GeneratorRating,
) -> RecursionStatus<Vec<u8>> {
    let mut groups: (ByteList, ByteList) = (vec![], vec![]);
    for item in list {
        if item[bit_index] == 0 {
            groups.0.push(item.clone());
        } else {
            groups.1.push(item.clone());
        }
    }
    let group = match generator_rating {
        GeneratorRating::CO2 => match (groups.0.len() <= groups.1.len()) {
            true => groups.0,
            false => groups.1,
        },
        GeneratorRating::OXYGEN => match groups.0.len() > groups.1.len() {
            true => groups.0,
            false => groups.1,
        },
    };
    if group.len() < 2 {
        RecursionStatus::DONE(group[0].clone())
    } else {
        RecursionStatus::CONTINUE(group)
    }
}

fn part2() -> i32 {
    println!("Part 2");
    let mut byte_list = RecursionStatus::CONTINUE(get_bits());
    let mut index = 0;
    while let RecursionStatus::CONTINUE(x) = byte_list {
        byte_list = filter_by_bit_index(&x, index, GeneratorRating::OXYGEN);
        index += 1;
    }
    let oxygen;
    if let RecursionStatus::DONE(x) = byte_list {
        oxygen = bit_list_to_number(x);
    } else {
        oxygen = 0;
    }

    let mut byte_list = RecursionStatus::CONTINUE(get_bits());
    let mut index = 0;
    while let RecursionStatus::CONTINUE(x) = byte_list {
        byte_list = filter_by_bit_index(&x, index, GeneratorRating::CO2);
        index += 1;
    }
    let co2;
    if let RecursionStatus::DONE(x) = byte_list {
        co2 = bit_list_to_number(x);
    } else {
        co2 = 0;
    }
    co2 * oxygen
}

pub fn exec() -> i32 {
    println!("Day 3");
    part2()
}
