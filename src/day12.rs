#![allow(unused)]
use std::collections::HashMap;

type CaveSystem<'a> = HashMap<&'a str, Vec<&'a str>>;

fn explore(system: &CaveSystem, cave_id: &str, path: &mut String, paths: &mut Vec<String>) {
    let mut path = path.clone();
    path.push_str(format!(",{}", cave_id).as_str());
    if (cave_id == "end") {
        paths.push(path);
        return;
    }
    let child_paths = system.get(cave_id).unwrap();
    for child_path in child_paths {
        // println!("{} -> {}", cave_id, child_path);
        if system.contains_key(child_path) {
            let mut new_paths = system.clone();
            if (cave_id.to_lowercase() == cave_id) {
                new_paths.remove(cave_id);
            }
            explore(&new_paths, &child_path, &mut path, paths);
        }
    }
}

fn part1() -> usize {
    let pairs = include_str!("../input/day12.txt")
        .lines()
        .map(|l| l.split_once("-").unwrap())
        .map(|(a, b)| {
            let mut paths = vec![(a, b)];
            paths.push((b, a));
            paths
        })
        .flatten();

    let mut cave_system: CaveSystem = HashMap::new();
    for (key, value) in pairs {
        let entry = cave_system.entry(key).or_insert(Vec::new());
        entry.push(value);
    }

    let mut paths = Vec::new();
    explore(&cave_system, "start", &mut String::new(), &mut paths);

    paths.len()
}

fn can_travel_to(path: &Vec<String>, cave_id: &str) -> bool {
    if (cave_id == "start") {
        return false;
    }
    let mut small_caves: Vec<&String> = path.iter().filter(|&c| c.to_lowercase() == *c).collect();
    if !small_caves.contains(&&cave_id.to_string()) {
        return true;
    }
    let original_size = small_caves.len();
    small_caves.sort();
    small_caves.dedup();
    original_size == small_caves.len()
}

fn explore2(system: &CaveSystem, cave_id: &str, path: &mut Vec<String>, paths: &mut Vec<String>) {
    let mut path = path.clone();
    path.push(cave_id.to_string());

    if (cave_id == "end") {
        paths.push(path.join(","));
        return;
    }

    let mut new_system = system.clone();
    let mut child_paths = new_system.get(cave_id).unwrap();
    let child_paths: Vec<&str> = child_paths
        .iter()
        .filter(|&p| can_travel_to(&path, p))
        .map(|p| *p)
        .collect();
    new_system.insert(cave_id, child_paths.clone());

    for child_path in child_paths {
        if new_system.contains_key(child_path) {
            // println!("{} -> {}", path.join(","), child_path);
            explore2(&new_system, &child_path, &mut path, paths);
        }
    }
}

fn part2() -> usize {
    let pairs = include_str!("../input/day12.txt")
        .lines()
        .map(|l| l.split_once("-").unwrap())
        .map(|(a, b)| {
            let mut paths = vec![(a, b)];
            paths.push((b, a));
            paths
        })
        .flatten();

    let mut cave_system: CaveSystem = HashMap::new();
    for (key, value) in pairs {
        let entry = cave_system.entry(key).or_insert(Vec::new());
        entry.push(value);
    }

    let mut paths = Vec::new();
    explore2(&cave_system, "start", &mut Vec::new(), &mut paths);

    paths.len()
}

pub fn exec() -> usize {
    println!("Day 12");
    part2()
}
