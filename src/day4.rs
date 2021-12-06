#![allow(unused)]

#[derive(Debug, Clone)]
struct Board {
    numbers: Vec<usize>,
}

impl Board {
    fn from(input: &Vec<usize>) -> Self {
        Self {
            numbers: input.clone(),
        }
    }

    fn rows(&self) -> Vec<&[usize]> {
        self.numbers.chunks(5).collect()
    }

    fn columns(&self) -> Vec<Vec<usize>> {
        let mut cols: Vec<Vec<usize>> = vec![];
        for i in 0..5 {
            let thing: Vec<usize> = self.numbers.iter().skip(i).step_by(5).copied().collect();
            cols.push(thing);
        }
        cols
    }

    fn has_bingo(&self, numbers: &[usize]) -> bool {
        self.rows()
            .iter()
            .any(|row| row.iter().all(|x| numbers.contains(x)))
            || self
                .columns()
                .iter()
                .any(|col| col.iter().all(|x| numbers.contains(x)))
    }

    fn get_unmarked_numbers(&self, marked_numbers: &Vec<usize>) -> Vec<usize> {
        self.numbers
            .iter()
            .filter(|x| !marked_numbers.contains(x))
            .copied()
            .collect()
    }
}

fn part1() -> usize {
    let (input_numbers, boards) = include_str!("../input/day4.txt")
        .split_once("\n\n")
        .unwrap();

    let input_numbers: Vec<usize> = input_numbers
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    let boards: Vec<Board> = boards
        .split("\n\n")
        .map(|b| {
            b.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .map(|b| Board::from(&b))
        .collect();

    let mut current_inputs = vec![];
    for input_number in input_numbers {
        current_inputs.push(input_number);
        for board in boards.clone() {
            if board.has_bingo(&current_inputs) {
                return board
                    .get_unmarked_numbers(&current_inputs)
                    .iter()
                    .copied()
                    .sum::<usize>()
                    * current_inputs.last().unwrap();
            }
        }
    }
    0
}

fn part2() -> usize {
    let (input_numbers, boards) = include_str!("../input/day4.txt")
        .split_once("\n\n")
        .unwrap();

    let input_numbers: Vec<usize> = input_numbers
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    let mut boards: Vec<Board> = boards
        .split("\n\n")
        .map(|b| {
            b.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .map(|b| Board::from(&b))
        .collect();

    let mut current_inputs = vec![];
    for input_number in input_numbers {
        current_inputs.push(input_number);
        if (boards.len() > 1) {
            boards.retain(|b| !b.has_bingo(&current_inputs));
            continue;
        }

        for board in boards.clone() {
            if board.has_bingo(&current_inputs) {
                return board
                    .get_unmarked_numbers(&current_inputs)
                    .iter()
                    .copied()
                    .sum::<usize>()
                    * current_inputs.last().unwrap();
            }
        }
    }
    0
}

pub fn exec() -> usize {
    println!("Day 3");
    part2()
}
