use std::collections::{BinaryHeap, HashMap};
use std::time::Instant;

// helper to check adjacent cells
const HELPER: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
const TILES: usize = 5;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Node {
    x: i32,
    y: i32,
    risk: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.risk.cmp(&self.risk)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input() -> Vec<Vec<i32>> {
    let mut input = include_str!("../input.txt").lines();
    let mut map = Vec::new();
    while let Some(line) = input.next() {
        let row: Vec<i32> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();

        map.push(row);
    }
    map
}

#[allow(dead_code)]
fn part1(map: Vec<Vec<i32>>) -> i32 {
    let start_time = Instant::now();
    let max = (map[0].len() - 1) as i32;
    let mut to_visit = BinaryHeap::new();
    let mut best_path: HashMap<(i32, i32), i32> = HashMap::new();
    to_visit.push(Node {
        x: 0,
        y: 0,
        risk: 0,
    });
    while !to_visit.is_empty() {
        let node = to_visit.pop().unwrap();
        if node.risk < *best_path.entry((node.x, node.y)).or_insert(i32::MAX) {
            best_path.insert((node.x, node.y), node.risk);
            for (hx, hy) in HELPER.iter() {
                let x = node.x + hx;
                let y = node.y + hy;
                if x < 0 || y < 0 || x > max || y > max {
                    continue;
                }
                let risk = node.risk + map[y as usize][x as usize];
                to_visit.push(Node { x, y, risk });
            }
        }
    }
    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("Time taken by the program: {:?}", elapsed_time);
    best_path[&(max, max)]
}

fn get_big_map(map: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut big_map = Vec::new();
    let size = map.len();
    for x in 0..TILES * size {
        let mut row: Vec<i32> = Vec::new();
        for y in 0..TILES * size {
            let new = (map[x % size][y % size] + (x / size) as i32 + (y / size) as i32 - 1) % 9 + 1;
            row.push(new);
        }
        big_map.push(row);
    }
    big_map
}

fn part2(map: Vec<Vec<i32>>) -> i32 {
    let map = get_big_map(map);
    let start_time = Instant::now();
    let max = (map[0].len() - 1) as i32;
    let mut to_visit = BinaryHeap::new();
    let mut best_path = HashMap::new();
    to_visit.push(Node {
        x: 0,
        y: 0,
        risk: 0,
    });
    while !to_visit.is_empty() {
        let node = to_visit.pop().unwrap();
        if node.risk < *best_path.entry((node.x, node.y)).or_insert(i32::MAX) {
            best_path.insert((node.x, node.y), node.risk);
            for (hx, hy) in HELPER.iter() {
                let x = node.x + hx;
                let y = node.y + hy;
                if x < 0 || y < 0 || x > max || y > max {
                    continue;
                }
                let risk = node.risk + map[y as usize][x as usize];
                to_visit.push(Node { x, y, risk });
            }
        }
    }
    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("Time taken by the program: {:?}", elapsed_time);
    best_path[&(max, max)]
}

fn main() {
    let map = parse_input();
    // println!("Part 1 = {}", part1(map));
    println!("Part 2 = {}", part2(map));
}
