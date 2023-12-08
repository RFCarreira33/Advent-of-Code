use std::{collections::HashMap, cmp};

const HAND_PRIORITY: [HandType; 7] = [
    HandType::Hc,
    HandType::Onep,
    HandType::Twop,
    HandType::Threeok,
    HandType::Fh,
    HandType::Fourok,
    HandType::Fiveok,
];

const CARD_PRIORITY: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J','Q', 'K', 'A'
];

const CARD_PRIORITY2: [char; 13] = [
    'J','2', '3', '4', '5', '6', '7', '8', '9', 'T','Q', 'K', 'A'
];

const NUM_CARDS: usize = 5;

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: u32,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum HandType {
    Fiveok,
    Fourok,
    Fh,
    Threeok,
    Twop,
    Onep,
    Hc,
}

impl HandType {
    fn check_handtype(chars: &Vec<char>, joker: bool) -> HandType {
        let mut jokers = 0;
        let mut map: HashMap<char, u32> = HashMap::new();
        chars.iter().for_each(|c| {
            if joker && *c == 'J' {
                jokers += 1;
                return;
            }
            map.entry(*c).and_modify(|cur| *cur += 1).or_insert(1);
        });
        if map.is_empty() || map.len() == 1 {
            return HandType::Fiveok;
        }
        let mut values: Vec<u32> = map.values().cloned().collect();
        values.sort();
        let len = values.len()-1;
        if joker {
            values[len] += jokers;
        }
        match (values[len-1], values[len]) {
            (1, 4) => HandType::Fourok,
            (2, 3) => HandType::Fh,
            (1, 3) => HandType::Threeok,
            (2, 2) => HandType::Twop,
            (1, 2) => HandType::Onep,
            (1, 1) => HandType::Hc,
            _ => unreachable!(),
        }
    }
}

fn parse_input(part2: bool) -> HashMap<HandType, Vec<Hand>> {
    let mut hands: HashMap<HandType, Vec<Hand>> = HashMap::new();
    std::fs::read_to_string("input.txt")
        .expect("Error reading file")
        .split_terminator("\n")
        .for_each(|line| {
            let (cards, bid) = line.split_at(5);
            let cards = cards.trim().chars().collect();
            let bid = bid.trim().parse::<u32>().unwrap();
            let entry = hands.entry(HandType::check_handtype(&cards, part2)).or_insert(vec![]);
            entry.push(Hand {
                cards,
                bid,
            });
        });
    hands
}

#[allow(dead_code)]
fn part1(mut input : HashMap<HandType, Vec<Hand>>) -> u32 {
    let mut winnings = 0;
    let mut rank = 1;
    HAND_PRIORITY.iter().for_each(|handtype| {
        input.get_mut(handtype).iter_mut().for_each(|hands| {
            hands.sort_by(|a, b| {
                for i in 0..NUM_CARDS {
                    let a = CARD_PRIORITY.iter().position(|&x| x == a.cards[i]).unwrap();
                    let b = CARD_PRIORITY.iter().position(|&x| x == b.cards[i]).unwrap();
                    if a != b {
                        return a.cmp(&b);
                    }
                }
                return cmp::Ordering::Equal;
            });
            hands.iter().for_each(|hand| {
                winnings += hand.bid * rank;
                rank += 1;
            })
        });
    });
    winnings
}

fn part2(mut input : HashMap<HandType, Vec<Hand>>) -> u32 {
    let mut winnings = 0;
    let mut rank = 1;
    HAND_PRIORITY.iter().for_each(|handtype| {
        input.get_mut(handtype).iter_mut().for_each(|hands| {
            hands.sort_by(|a, b| {
                for i in 0..NUM_CARDS {
                    let a = CARD_PRIORITY2.iter().position(|&x| x == a.cards[i]).unwrap();
                    let b = CARD_PRIORITY2.iter().position(|&x| x == b.cards[i]).unwrap();
                    if a != b {
                        return a.cmp(&b);
                    }
                }
                return cmp::Ordering::Equal;
            });
            hands.iter().for_each(|hand| {
                winnings += hand.bid * rank;
                rank += 1;
            })
        });
    });
    winnings
}

fn main() {
    let input: HashMap<HandType, Vec<Hand>> = parse_input(true);
    // println!("Part 1 = {}", part1(input));
    println!("Part 2 = {}", part2(input));
}


