#![allow(unused)]

fn get_adjacent_indexes(state: &Vec<u32>, index: usize) -> Vec<usize> {
    let size = state.len();
    let width = (size as f64).sqrt() as usize;
    let height = width;
    let row = (index / width);
    let column = (index % width);

    let mut indexes = vec![];
    if column > 0 {
        indexes.push(index - 1);
    }
    if column < width - 1 {
        indexes.push(index + 1);
    }
    if row > 0 {
        let prev_row_index = index - width;
        if column > 0 {
            indexes.push(prev_row_index - 1);
        }
        indexes.push(prev_row_index);
        if column < width - 1 {
            indexes.push(prev_row_index + 1);
        }
    }
    if row < height - 1 {
        let next_row_index = index + width;
        if column > 0 {
            indexes.push(next_row_index - 1);
        }
        indexes.push(next_row_index);
        if column < width - 1 {
            indexes.push(next_row_index + 1);
        }
    }
    indexes
}

fn part1() -> usize {
    let mut state: Vec<u32> = include_str!("../input/day11.txt")
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .flatten()
        .collect();

    let mut total_flashes = 0;
    for epoch in 0..100 {
        state.iter_mut().for_each(|count: &mut u32| *count += 1);
        loop {
            let new_state = state.clone();
            let flashes: Vec<(usize, &u32)> = new_state
                .iter()
                .enumerate()
                .filter(|(_, &count)| count == 10)
                .collect();
            total_flashes += flashes.len();

            if flashes.is_empty() {
                break;
            }

            for (index, count) in flashes {
                state[index] = 0;
                let indexes = get_adjacent_indexes(&state, index);
                for i in indexes {
                    if state[i] != 0 && state[i] < 10 {
                        state[i] += 1;
                    }
                }
            }
        }
    }

    total_flashes
}

fn part2() -> usize {
    let mut state: Vec<u32> = include_str!("../input/day11.txt")
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .flatten()
        .collect();

    let mut sync_epoch = 0;
    for epoch in 1..=1000 {
        let mut total_flashes = 0;
        state.iter_mut().for_each(|count: &mut u32| *count += 1);
        loop {
            let new_state = state.clone();
            let flashes: Vec<(usize, &u32)> = new_state
                .iter()
                .enumerate()
                .filter(|(_, &count)| count == 10)
                .collect();
            total_flashes += flashes.len();

            if flashes.is_empty() {
                break;
            }

            for (index, count) in flashes {
                state[index] = 0;
                let indexes = get_adjacent_indexes(&state, index);
                for i in indexes {
                    if state[i] != 0 && state[i] < 10 {
                        state[i] += 1;
                    }
                }
            }
        }
        if total_flashes == state.len() {
            sync_epoch = epoch;
            break;
        }
    }

    sync_epoch
}

pub fn exec() -> usize {
    println!("Day 11");
    part2()
}
