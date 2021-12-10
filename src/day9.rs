#![allow(unused)]
use std::collections::HashSet;

fn part1() -> usize {
    let lines: Vec<&str> = include_str!("../input/day9.txt").lines().collect();
    let height = lines.len();
    let width = lines[0].len();
    let depths: Vec<u32> = lines
        .iter()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()))
        .flatten()
        .collect();
    let lowest_depths: Vec<&u32> = depths
        .iter()
        .enumerate()
        .filter(|&(index, depth)| {
            let index = index as isize;
            let mut adjacent_depths = vec![];
            let mut indexes = vec![index + 1, index + (width as isize)];
            if (index - 1) >= 0 {
                indexes.push(index - 1);
            }
            if (index - (width as isize)) >= 0 {
                indexes.push(index - (width as isize));
            }
            for index in indexes {
                match depths.get(index as usize) {
                    Some(x) => adjacent_depths.push(x),
                    None => {}
                }
            }
            depth < adjacent_depths.iter().min().unwrap()
        })
        .map(|(_, d)| d)
        .collect();
    println!("{}x{}", width, height);
    println!("{:?}", lowest_depths);
    lowest_depths.into_iter().fold(0, |acc, &x| acc + x + 1) as usize
}

struct DepthChart {
    depths: Vec<u32>,
    width: usize,
    height: usize,
}

impl DepthChart {
    fn from(lines: Vec<&str>) -> Self {
        let height = lines.len();
        let width = lines[0].len();
        let depths: Vec<u32> = lines
            .iter()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()))
            .flatten()
            .collect();

        DepthChart {
            depths,
            width,
            height,
        }
    }

    fn adjacent_indexes(&self, index: usize) -> Vec<usize> {
        let size = self.depths.len();
        let mut indexes = vec![];
        let left = index as isize - 1;
        let top = index as isize - self.width as isize;
        if left >= 0 && index % self.width != 0 {
            indexes.push(left as usize);
        }
        if top >= 0 {
            indexes.push(top as usize);
        }
        if (index + 1 < size) && index % self.width != self.width - 1 {
            indexes.push(index + 1);
        }
        if (index + self.width < size) {
            indexes.push(index + self.width);
        }
        indexes
    }

    fn deepest_indexes(&self) -> Vec<usize> {
        self.depths
            .iter()
            .enumerate()
            .filter(|&(index, depth)| {
                let mut adjacent_depths = vec![];
                for index in self.adjacent_indexes(index) {
                    let d = self.depths.get(index).unwrap();
                    adjacent_depths.push(d);
                }
                depth < adjacent_depths.iter().min().unwrap()
            })
            .map(|(index, _)| index)
            .collect()
    }

    fn explore(&self, index: usize, explored: &mut HashSet<usize>) -> usize {
        explored.insert(index);
        let depth = self.depths.get(index).unwrap();
        let adjacent_indexes = self.adjacent_indexes(index);
        let adjacent_indexes: Vec<&usize> = adjacent_indexes
            .iter()
            .filter(|&i| !explored.contains(i))
            .collect();

        if depth >= &9 {
            return 0;
        }

        let mut count = 0;
        for adjacent_index in adjacent_indexes {
            if !explored.contains(&adjacent_index) {
                count += self.explore(*adjacent_index, explored);
            }
        }
        count + 1
    }
}

fn part2() -> usize {
    let lines: Vec<&str> = include_str!("../input/day9.txt").lines().collect();
    let depth_chart = DepthChart::from(lines);

    let mut sizes: Vec<usize> = depth_chart
        .deepest_indexes()
        .iter()
        .map(|index| depth_chart.explore(*index, &mut HashSet::new()))
        .collect();
    sizes.sort();
    sizes.iter().rev().take(3).fold(1, |acc, count| acc * count)
}

pub fn exec() -> usize {
    println!("Day 9");
    part2()
}
