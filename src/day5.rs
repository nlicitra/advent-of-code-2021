#![allow(unused)]
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn from_str(input: &str) -> Point {
        let (x, y) = input.split_once(",").unwrap();
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Line(Point, Point);
impl Line {
    fn from_str(input: &str) -> Self {
        let (start, end) = input.split_once(" -> ").unwrap();
        Self(Point::from_str(start), Point::from_str(end))
    }

    fn get_continuous_points(&self) -> Vec<Point> {
        let x_range: Vec<usize> = match self.0.x > self.1.x {
            false => (self.0.x..=self.1.x).collect(),
            true => (self.1.x..=self.0.x).rev().collect(),
        };

        let y_range: Vec<usize> = match self.0.y > self.1.y {
            false => (self.0.y..=self.1.y).collect(),
            true => (self.1.y..=self.0.y).rev().collect(),
        };

        if (self.0.y == self.1.y) {
            return (x_range.iter())
                .map(|(&x)| Point { x, y: y_range[0] })
                .collect();
        } else if (self.0.x == self.1.x) {
            return (y_range.iter())
                .map(|(&y)| Point { x: x_range[0], y })
                .collect();
        }
        (x_range.into_iter())
            .zip(y_range.into_iter())
            .map(|(x, y)| Point { x, y })
            .collect()
    }
}

fn part1() -> usize {
    let lines = include_str!("../input/day5.txt")
        .lines()
        .map(|l| Line::from_str(l))
        .filter(|l| (l.0.x == l.1.x) || (l.0.y == l.1.y))
        .collect::<Vec<Line>>();

    let points: Vec<Point> = lines
        .into_iter()
        .map(|l| l.get_continuous_points())
        .flatten()
        .collect();

    let mut point_map: HashMap<Point, usize> = HashMap::new();
    for point in points {
        let count = match point_map.get(&point) {
            Some(v) => v,
            None => &0,
        };
        point_map.insert(point, count + 1);
    }
    point_map.values().filter(|&v| *v >= 2).count()
}

fn part2() -> usize {
    let lines = include_str!("../input/day5.txt")
        .lines()
        .map(|l| Line::from_str(l))
        .collect::<Vec<Line>>();

    let points: Vec<Point> = lines
        .into_iter()
        .map(|l| l.get_continuous_points())
        .flatten()
        .collect();

    let mut point_map: HashMap<Point, usize> = HashMap::new();
    for point in points {
        let count = match point_map.get(&point) {
            Some(v) => v,
            None => &0,
        };
        point_map.insert(point, count + 1);
    }
    point_map.values().filter(|&v| *v >= 2).count()
}

pub fn exec() -> usize {
    println!("Day 5");
    part2()
}
