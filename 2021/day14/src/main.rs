use std::collections::HashMap;

const MAX_STEPS_1: usize = 10;
const MAX_STEPS_2: usize = 40;

type Pair = (char, char);

fn parse_input() -> (HashMap<Pair, char>, Vec<char>) {
    let mut input = include_str!("../input.txt").lines();
    let mut patterns: HashMap<Pair, char> = HashMap::new();
    let start: Vec<char> = input.next().unwrap().chars().collect();
    while let Some(line) = input.next() {
        let (pat, new) = match line.split_once(" -> ") {
            Some((pat, new)) => (pat, new),
            None => continue,
        };
        patterns.insert(
            // pattern
            (pat.chars().nth(0).unwrap(), pat.chars().nth(1).unwrap()),
            // char
            new.chars().nth(0).unwrap(),
        );
    }
    (patterns, start)
}

fn convert_start(start: Vec<char>, letter_count: &mut HashMap<char, i64>) -> HashMap<Pair, i64> {
    let mut polymer_pairs: HashMap<Pair, i64> = HashMap::new();
    *letter_count.entry(start[0]).or_insert(0) += 1;
    for i in 0..start.len() - 1 {
        *polymer_pairs.entry((start[i], start[i + 1])).or_insert(0) += 1;
        *letter_count.entry(start[i + 1]).or_insert(0) += 1;
    }
    polymer_pairs
}

fn step(
    patterns: &HashMap<Pair, char>,
    polymer_pairs: &HashMap<Pair, i64>,
    letter_count: &mut HashMap<char, i64>,
) -> HashMap<Pair, i64> {
    let mut new_polymer_pairs: HashMap<Pair, i64> = HashMap::new();
    for (pair, count) in polymer_pairs {
        match patterns.get(pair) {
            Some(new) => {
                new_polymer_pairs
                    .entry((pair.0, *new))
                    .and_modify(|e| *e += *count)
                    .or_insert(*count);

                new_polymer_pairs
                    .entry((*new, pair.1))
                    .and_modify(|e| *e += *count)
                    .or_insert(*count);

                letter_count
                    .entry(*new)
                    .and_modify(|e| *e += *count as i64)
                    .or_insert(*count as i64);
            }
            None => {
                new_polymer_pairs
                    .entry(*pair)
                    .and_modify(|e| *e += *count)
                    .or_insert(*count);
            }
        }
    }
    new_polymer_pairs
}

#[allow(dead_code)]
fn part1(patterns: HashMap<Pair, char>, polymer: Vec<char>) -> i64 {
    let mut letter_count: HashMap<char, i64> = HashMap::new();
    let mut pair_count = convert_start(polymer, &mut letter_count);
    for _ in 0..MAX_STEPS_1 {
        pair_count = step(&patterns, &mut pair_count, &mut letter_count);
    }
    *letter_count.values().max().unwrap() - *letter_count.values().min().unwrap()
}

fn part2(patterns: HashMap<Pair, char>, polymer: Vec<char>) -> i64 {
    let mut letter_count: HashMap<char, i64> = HashMap::new();
    let mut pair_count = convert_start(polymer, &mut letter_count);
    for _ in 0..MAX_STEPS_2 {
        pair_count = step(&patterns, &mut pair_count, &mut letter_count);
    }
    *letter_count.values().max().unwrap() - *letter_count.values().min().unwrap()
}

fn main() {
    let (map, polymer) = parse_input();
    // println!("Part 1 = {}", part1(map, polymer));
    println!("Part 2 = {}", part2(map, polymer));
}
