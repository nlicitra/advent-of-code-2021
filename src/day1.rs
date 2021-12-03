#![allow(unused)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_depth_values() -> Vec<u32> {
    read_lines("./input/day1.txt")
        .expect("Failed to open input file: ./input/day1.txt")
        .into_iter()
        .map(|l| l.unwrap().parse().unwrap())
        .collect()
}

fn part1() -> u32 {
    println!("Part 1");
    let mut last_depth = 0;
    let mut counter = 0;
    let depths = get_depth_values();
    for depth in depths {
        if depth > last_depth {
            counter += 1;
        }
        last_depth = depth;
    }
    match counter > 0 {
        true => counter - 1,
        false => 0,
    }
}

fn part2() -> u32 {
    println!("Part 2");
    let mut last_depth = 0;
    let mut counter = 0;
    let depths = get_depth_values();
    for index in 2..(depths.len()) {
        let frame = &depths[(index - 2)..(index + 1)];
        let depth = frame.iter().sum();
        if depth > last_depth {
            counter += 1;
        }
        last_depth = depth;
    }

    match counter > 0 {
        true => counter - 1,
        false => 0,
    }
}

pub fn exec() -> u32 {
    println!("Day 1");
    part2()
}
