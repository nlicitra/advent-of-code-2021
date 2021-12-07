#![allow(unused)]
use std::cmp::Ordering;

fn part1() -> usize {
    let mut positions = include_str!("../input/day7.txt")
        .split(",")
        .map(|n| {
            n.parse()
                .expect(format!("{} is not a valid number", n).as_str())
        })
        .collect::<Vec<i32>>();
    let max = *positions.iter().max().unwrap();

    let min: i32 = (0..max)
        .map(|p| {
            positions
                .clone()
                .into_iter()
                .map(|position| (position - p).abs())
                .sum()
        })
        .min()
        .unwrap();
    min as usize
}

fn part2() -> usize {
    let mut positions = include_str!("../input/day7.txt")
        .split(",")
        .map(|n| {
            n.parse()
                .expect(format!("{} is not a valid number", n).as_str())
        })
        .collect::<Vec<i32>>();
    let max = *positions.iter().max().unwrap();

    let min: i32 = (0..max)
        .map(|p| {
            positions
                .clone()
                .into_iter()
                .map(|position| {
                    let distance = (position - p).abs();
                    let fuel: i32 = (1..=distance).sum();
                    fuel
                })
                .sum()
        })
        .min()
        .unwrap();
    min as usize
}

pub fn exec() -> usize {
    println!("Day 7");
    part2()
}
