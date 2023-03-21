use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Cave {
    name: String,
    small: bool,
    connections: HashSet<String>,
}

impl Cave {
    fn new(name: String) -> Cave {
        Cave {
            name: name.clone(),
            small: Cave::is_small(name),
            connections: HashSet::new(),
        }
    }

    fn is_small(s: String) -> bool {
        s.to_lowercase() == s
    }
}

fn parse_input() -> HashMap<String, Cave> {
    let mut input = include_str!("../input.txt").lines();
    let mut caves: HashMap<String, Cave> = HashMap::new();
    while let Some(line) = input.next() {
        let (from, to) = line.split_once("-").unwrap();
        // insert caves if they don't exist
        caves
            .entry(from.to_string())
            .or_insert(Cave::new(from.to_string()));

        caves
            .entry(to.to_string())
            .or_insert(Cave::new(to.to_string()));

        // update connections
        caves
            .get_mut(from)
            .unwrap()
            .connections
            .insert(to.to_string());
        caves
            .get_mut(to)
            .unwrap()
            .connections
            .insert(from.to_string());
    }
    caves
}

fn recursive(
    caves: HashMap<String, Cave>,
    from: Cave,
    mut path: Vec<Cave>,
    mut visited: HashSet<String>,
    dup_cave: Option<bool>,
) -> i32 {
    path.push(from.clone());
    if from.name == "end" {
        return 1;
    }

    let mut dup = dup_cave;
    let mut count = 0;
    if from.small {
        if visited.contains(&from.name) {
            if dup.is_some() && dup.unwrap() {
                dup = Some(false);
            } else {
                return 0;
            }
        }
        visited.insert(from.name);
    }
    for c in from.connections {
        if c == "start" {
            continue;
        }
        // This could be a lot more efficient
        // but I struggled with this day so yeah
        count += recursive(
            caves.clone(),
            caves.get(&c).unwrap().clone(),
            path.clone(),
            visited.clone(),
            dup,
        );
    }
    count
}

#[allow(dead_code)]
fn part1(input: HashMap<String, Cave>) -> i32 {
    recursive(
        input.clone(),
        input.get("start").unwrap().clone(),
        vec![],
        HashSet::new(),
        None,
    )
}

fn part2(input: HashMap<String, Cave>) -> i32 {
    recursive(
        input.clone(),
        input.get("start").unwrap().clone(),
        vec![],
        HashSet::new(),
        Some(true),
    )
}

fn main() {
    let input: HashMap<String, Cave> = parse_input();
    // println!("Part 1 = {}", part1(input));
    println!("Part 2 = {}", part2(input));
}
