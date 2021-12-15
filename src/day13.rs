#![allow(unused)]
use std::fmt;

struct Grid {
    items: Vec<bool>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(init_points: Vec<(usize, usize)>) -> Self {
        let (max_x, max_y) = init_points.iter().fold((0, 0), |acc, p| {
            let max_x = if p.0 > acc.0 { p.0 } else { acc.0 };
            let max_y = if p.1 > acc.1 { p.1 } else { acc.1 };
            (max_x, max_y)
        });

        let width = max_x + 1;
        let height = max_y + 1;
        let mut items = vec![false; width * height];
        let mut grid = Self {
            items,
            width,
            height,
        };
        init_points
            .iter()
            .for_each(|(x, y)| grid.set_item(*x, *y, true));
        grid
    }

    fn set_item(&mut self, x: usize, y: usize, val: bool) {
        let index = y * self.width + x;
        self.items[index] = val;
    }

    fn fold_on_y(&mut self, y: usize) {
        let new_items = self.items.clone();
        let rows: Vec<&[bool]> = new_items.chunks(self.width).collect();
        let (first, second) = rows.split_at(y);
        let first: Vec<Vec<bool>> = first.iter().map(|x| x.to_vec()).collect();
        let second: Vec<Vec<bool>> = second.iter().map(|x| x.to_vec()).rev().collect();
        let second = second.split_last().unwrap().1.to_vec();
        // println!("{:?}x{:?}", first, second);
        let (mut big, mut small) = if first.len() >= second.len() {
            (first.to_vec(), second)
        } else {
            (second, first.to_vec())
        };
        let diff = big.len() - small.len();

        for (index, row) in big.iter_mut().enumerate() {
            let mut default_row = Vec::with_capacity(self.width);
            default_row.fill(false);
            let other_row = match index >= diff {
                true => small.get(index - diff).unwrap_or(&default_row),
                false => &default_row,
            };
            for (i, item) in row.iter_mut().enumerate() {
                *item = *item || *other_row.get(i).unwrap_or(&false);
            }
        }
        self.items = big.iter().flatten().map(|b| *b).collect();
        self.height = self.items.iter().count() / self.width;
    }

    fn get_column(&self, column_index: usize) -> Vec<bool> {
        (0..self.items.len())
            .step_by(self.width)
            .map(|i| *self.items.get(i + column_index).unwrap())
            .collect()
    }

    fn fold_on_x(&mut self, x: usize) {
        let new_items = self.items.clone();
        let rows: Vec<&[bool]> = new_items.chunks(self.width).collect();
        let new_state = rows
            .iter()
            .map(|&row| {
                let (first, second) = row.split_at(x);
                let second: Vec<bool> = second.iter().map(|x| *x).rev().collect();
                let second = second.split_last().unwrap().1.to_vec();
                // println!("{:?}x{:?}", first, second);
                let (mut big, mut small) = if first.len() >= second.len() {
                    (first.to_vec(), second)
                } else {
                    (second, first.to_vec())
                };
                let diff = big.len() - small.len();

                for (index, item) in big.iter_mut().enumerate().rev() {
                    *item = *item || *small.get(index - diff).unwrap_or(&false);
                }
                big.clone()
            })
            .flatten()
            .collect();
        self.items = new_state;
        self.width = self.items.len() / self.height;
    }

    fn visible_count(&self) -> usize {
        self.items.iter().filter(|&x| *x).count()
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = self
            .items
            .chunks(self.width)
            .map(|c| {
                c.iter().fold(String::new(), |mut acc, i| {
                    let token = if *i { "#" } else { "." };
                    acc.push_str(token);
                    acc
                })
            })
            .collect::<Vec<String>>()
            .join("\n");
        f.write_str(format!("{}\n{}x{}", string.as_str(), self.width, self.height).as_str())
    }
}

fn part1() -> usize {
    let (points, folds) = include_str!("../input/day13.txt")
        .split_once("\n\n")
        .unwrap();
    let points: Vec<(usize, usize)> = points
        .split_whitespace()
        .map(|p| {
            let (x, y) = p.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let mut grid = Grid::new(points);
    grid.fold_on_x(655);
    grid.visible_count()
}

fn part2() -> usize {
    let (points, folds) = include_str!("../input/day13.txt")
        .split_once("\n\n")
        .unwrap();
    let points: Vec<(usize, usize)> = points
        .split_whitespace()
        .map(|p| {
            let (x, y) = p.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();
    // println!("{:?}", points);

    let mut grid = Grid::new(points);
    // println!("{:?}\n\n", grid);
    folds.split("\n").for_each(|l| {
        println!("{:?}", l);
        let command = l.split_once("fold along ").unwrap().1;
        let (axis, index) = command.split_once("=").unwrap();
        let index = index.parse().unwrap();
        if axis == "x" {
            grid.fold_on_x(index);
        } else {
            grid.fold_on_y(index);
        }
    });
    println!("{:?}", grid);
    grid.visible_count()
}

pub fn exec() -> usize {
    println!("Day 13");
    part2()
}
