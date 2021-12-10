#![allow(unused)]

struct Chunk {
    open: char,
    close: char,
    points: usize,
    complete_points: usize,
}

impl Chunk {
    fn from(token: &char) -> Self {
        match token {
            '(' | ')' => Chunk {
                open: '(',
                close: ')',
                points: 3,
                complete_points: 1,
            },
            '[' | ']' => Chunk {
                open: '[',
                close: ']',
                points: 57,
                complete_points: 2,
            },
            '{' | '}' => Chunk {
                open: '{',
                close: '}',
                points: 1197,
                complete_points: 3,
            },
            '<' | '>' => Chunk {
                open: '<',
                close: '>',
                points: 25137,
                complete_points: 4,
            },
            _ => panic!("Unexpected token."),
        }
    }
}

fn validate_line(tokens: &Vec<char>) -> usize {
    let mut chunks: Vec<Chunk> = vec![];
    for token in tokens {
        let chunk = Chunk::from(token);
        if chunk.open == *token {
            chunks.push(chunk);
            continue;
        }
        if (chunk.close == *token) {
            let prev_chunk = chunks.pop();
            if prev_chunk.is_none() {
                return chunk.points;
            }
            if chunk.close != prev_chunk.unwrap().close {
                return chunk.points;
            }
            continue;
        }
        return chunk.points;
    }
    0
}

fn complete_line(tokens: &Vec<char>) -> usize {
    let mut chunks: Vec<Chunk> = vec![];
    let mut missing_chunks: Vec<Chunk> = vec![];
    for token in tokens {
        let chunk = Chunk::from(token);
        if chunk.open == *token {
            chunks.push(chunk);
            continue;
        }
        if (chunk.close == *token) {
            let prev_chunk = chunks.pop();
            if prev_chunk.is_none() {
                panic!("We shouldn't have an early closing bracket")
            }
            if chunk.close != prev_chunk.unwrap().close {
                panic!("We shouldn't have an incorrect closing bracket")
            }
            continue;
        }
        panic!("Something bad happened");
    }
    chunks
        .iter()
        .rev()
        .fold(0, |acc, c| (5 * acc) + c.complete_points)
}

fn part1() -> usize {
    let chunk_list: Vec<Vec<char>> = include_str!("../input/day10.txt")
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    chunk_list.iter().map(|x| validate_line(x)).sum()
}

fn part2() -> usize {
    let incompletes: Vec<Vec<char>> = include_str!("../input/day10.txt")
        .lines()
        .map(|l| l.chars().collect())
        .filter(|tokens| validate_line(tokens) == 0)
        .collect();
    let mut scores: Vec<usize> = incompletes
        .iter()
        .map(|tokens| complete_line(tokens))
        .collect();

    scores.sort();

    let median_index = scores.len() / 2;
    *scores.get(median_index).unwrap()
}

pub fn exec() -> usize {
    println!("Day 10");
    part2()
}
