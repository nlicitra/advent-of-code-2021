#![allow(unused)]
use std::collections::HashMap;

fn part1() -> usize {
    let outputs = include_str!("../input/day8.txt")
        .lines()
        .map(|l| l.split_once(" | ").unwrap().1)
        .map(|outputs| outputs.split(" ").collect::<Vec<&str>>())
        .flatten()
        .filter(|&o| [2, 3, 4, 7].contains(&o.len()))
        .collect::<Vec<&str>>();
    println!("{:?}", outputs);
    outputs.len()
}

fn part2() -> usize {
    // This is by far some of the worst code I have ever written.
    let signal_index_list = vec![
        vec![0, 1, 2, 4, 5, 6],    // 0
        vec![2, 5],                // 1
        vec![0, 2, 3, 4, 6],       // 2
        vec![0, 2, 3, 5, 6],       // 3
        vec![1, 2, 3, 5],          // 4
        vec![0, 1, 3, 5, 6],       // 5
        vec![0, 1, 3, 4, 5, 6],    // 6
        vec![0, 2, 5],             // 7
        vec![0, 1, 2, 3, 4, 5, 6], // 8
        vec![0, 1, 2, 3, 5, 6],    // 9
    ];
    include_str!("../input/day8.txt")
        .lines()
        .map(|l| {
            let (signal_patterns, output) = l.split_once(" | ").unwrap();
            let map =
                signal_patterns
                    .split_whitespace()
                    .fold(HashMap::new(), |mut acc, pattern| {
                        let entry = acc.entry(pattern.len()).or_insert(Vec::new());
                        entry.push(pattern);
                        acc
                    });

            let mut signal_map = HashMap::new();
            let one_digit_signals = map.get(&2).unwrap()[0].chars().collect::<Vec<char>>();
            // let top_right_signal = one_digit_signals.into_iter().find(|x| map.get(&)).unwrap();
            signal_map.insert(one_digit_signals[0], 2);
            signal_map.insert(one_digit_signals[1], 5);

            let seven_digit_signals = map.get(&3).unwrap()[0].chars().collect::<Vec<char>>();
            let top_signal = seven_digit_signals
                .into_iter()
                .find(|c| !signal_map.contains_key(c))
                .unwrap();
            signal_map.insert(top_signal, 0);

            let four_digit_signals = map.get(&4).unwrap()[0].chars().collect::<Vec<char>>();
            let missing_signals = four_digit_signals
                .into_iter()
                .filter(|c| !signal_map.contains_key(c))
                .collect::<Vec<char>>();

            let found_signals = map
                .get(&5)
                .unwrap()
                .into_iter()
                .map(|p| p.chars())
                .flatten()
                .fold(HashMap::new(), |mut acc, c| {
                    let entry = acc.entry(c).or_insert(0);
                    *entry += 1;
                    acc
                })
                .into_iter()
                .filter(|(k, v)| v == &1)
                .map(|(k, v)| k)
                .collect::<Vec<char>>();
            let top_left_signal = found_signals
                .iter()
                .find(|c| missing_signals.contains(c))
                .unwrap();
            signal_map.insert(*top_left_signal, 1);

            let middle_signal = missing_signals
                .into_iter()
                .find(|c| !signal_map.contains_key(c))
                .unwrap();
            signal_map.insert(middle_signal, 3);

            let bottom_left_signal = found_signals
                .into_iter()
                .find(|c| !signal_map.contains_key(c))
                .unwrap();
            signal_map.insert(bottom_left_signal, 4);

            let bottom_signal = map.get(&7).unwrap()[0]
                .chars()
                .find(|c| !signal_map.contains_key(c))
                .unwrap();
            signal_map.insert(bottom_signal, 6);

            let char_count = map
                .values()
                .flatten()
                .map(|x| *x)
                .collect::<Vec<&str>>()
                .join("")
                .chars()
                .fold(HashMap::new(), |mut acc, c| {
                    let entry = acc.entry(c).or_insert(0);
                    *entry += 1;
                    acc
                });

            // top 8
            // top-left 6
            // top-right 8
            // middle 7
            // bottom-left 4
            // bottom-right 9
            // bottom 7
            let (top_left, _) = char_count.iter().find(|(_, &x)| x == 6).unwrap();
            signal_map.insert(*top_left, 1);
            let (bottom_left, _) = char_count.iter().find(|(_, &x)| x == 4).unwrap();
            signal_map.insert(*bottom_left, 4);
            let (bottom_right, _) = char_count.iter().find(|(_, &x)| x == 9).unwrap();
            if (signal_map.get(bottom_right).unwrap() != &5) {
                let (old_char, _) = signal_map
                    .clone()
                    .into_iter()
                    .find(|(_, v)| *v == 5)
                    .unwrap();
                signal_map.insert(*bottom_right, 5).unwrap();
                signal_map.insert(old_char, 2);
            }

            // let top_or_top_right = char_count
            //     .iter()
            //     .filter(|(_, &x)| x == 8)
            //     .map(|(&k, _)| k)
            //     .collect::<Vec<char>>();

            // let middle_or_bottom = char_count
            //     .iter()
            //     .filter(|(_, &x)| x == 7)
            //     .map(|(&k, _)| k)
            //     .collect::<Vec<char>>();

            // if char_count.get(map.get(&2).unwrap()[0]).unwrap()
            //     > char_count.get(map.get(&5).unwrap()[0]).unwrap()
            // {
            //     let old_val = map.get(&2);
            // }

            let number: usize = output
                .split_whitespace()
                .map(|o| {
                    let mut indexes = o
                        .chars()
                        .map(|c| *signal_map.get(&c).unwrap())
                        .collect::<Vec<i32>>();
                    indexes.sort();
                    signal_index_list
                        .clone()
                        .into_iter()
                        .enumerate()
                        .find(|(i, list)| list.eq(&indexes))
                        .unwrap()
                        .0
                        .to_string()
                })
                .collect::<Vec<String>>()
                .join("")
                .parse()
                .unwrap();

            // println!("{:?}", number);
            number
        })
        .sum()
}

pub fn exec() -> usize {
    println!("Day 8");
    part2()
}
