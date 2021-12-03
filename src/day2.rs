#![allow(unused)]

use std::fmt::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

enum Direction {
    UP,
    DOWN,
    FORWARD,
}

struct PositionDelta {
    position: i32,
    depth: i32,
}

struct Orientation {
    position: i32,
    depth: i32,
    aim: i32,
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = match s.to_uppercase().as_str() {
            "UP" => Direction::UP,
            "DOWN" => Direction::DOWN,
            "FORWARD" => Direction::FORWARD,
            d => panic!("Unexpected direction: {}", d),
        };
        Ok(direction)
    }
}
struct Command {
    direction: Direction,
    distance: i32,
}

impl Command {
    fn get_position_delta(&self) -> PositionDelta {
        match self.direction {
            Direction::FORWARD => PositionDelta {
                position: self.distance,
                depth: 0,
            },
            Direction::UP => PositionDelta {
                position: 0,
                depth: -(self.distance),
            },
            Direction::DOWN => PositionDelta {
                position: 0,
                depth: self.distance,
            },
        }
    }

    fn get_next_position(&self, orientation: Orientation) -> Orientation {
        match self.direction {
            Direction::FORWARD => Orientation {
                position: orientation.position + self.distance,
                depth: orientation.depth + (orientation.aim * self.distance),
                aim: orientation.aim,
            },
            Direction::UP => Orientation {
                position: orientation.position,
                depth: orientation.depth,
                aim: orientation.aim - self.distance,
            },
            Direction::DOWN => Orientation {
                position: orientation.position,
                depth: orientation.depth,
                aim: orientation.aim + self.distance,
            },
        }
    }
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        let direction = Direction::from_str(parts[0]).unwrap();
        let distance: i32 = parts[1].parse().unwrap();
        Ok(Command {
            direction,
            distance,
        })
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_commands() -> Vec<Command> {
    let input_path = "./input/day2.txt";
    read_lines(input_path)
        .expect(format!("Failed to open input file: {}", input_path).as_str())
        .into_iter()
        .map(|l| Command::from_str(l.unwrap().as_str()).unwrap())
        .collect()
}

fn part1() -> i32 {
    println!("Part 1");
    let mut position = 0;
    let mut depth = 0;
    let commands = get_commands();
    for command in commands {
        let delta = command.get_position_delta();
        position += delta.position;
        depth += delta.depth;
    }
    position * depth
}

fn part2() -> i32 {
    println!("Part 2");
    let mut orientation = Orientation {
        aim: 0,
        position: 0,
        depth: 0,
    };
    let commands = get_commands();
    for command in commands {
        orientation = command.get_next_position(orientation);
    }
    orientation.position * orientation.depth
}

pub fn exec() -> i32 {
    println!("Day 2");
    part2()
}
