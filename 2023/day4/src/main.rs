use std::{collections::HashMap, usize};

const SIZE: usize = 219;

fn parse_input() -> (Vec<Vec<String>>, Vec<HashMap<u32, Vec<u32>>>) {
    let mut winning: Vec<Vec<String>> = vec![vec![];SIZE];
    let mut cards: Vec<HashMap<u32, Vec<u32>>> = vec![HashMap::new();SIZE];
    std::fs::read_to_string("input.txt")
        .expect("Error reading file")
        .split_terminator("\n")
        .enumerate().for_each(|(index, line)| line.split(":").skip(1)
            .for_each(|h| h.split("|").enumerate()
                .for_each(|(i, sides)| sides.split_whitespace()
                    .for_each(|number| {
                        let int_number = number.parse::<u32>().unwrap();
                        if i == 0 {
                            winning[index].push(number.to_owned());
                            return;
                        } 
                        let mut map_indice:u32 = 0;
                        if number.len() == 2 {
                            map_indice = number.chars().next().unwrap().to_digit(10).unwrap();
                        }
                        cards[index]
                            .entry(map_indice)
                            .or_insert_with(Vec::new)
                            .push(int_number);
                    }))));
    (winning, cards)
}

#[allow(dead_code)]
fn part1(input: Vec<HashMap<u32, Vec<u32>>>, winning: Vec<Vec<String>>) -> u32 {
    let mut total = 0;
    winning.iter().enumerate().for_each(|(index, card)| {
        let mut card_total = 0;
        card.iter().for_each(|winning| {
            let mut map_indice:u32 = 0;
            if winning.len() == 2 {
                map_indice = winning.chars().next().unwrap().to_digit(10).unwrap();
            }
            if let Some(values) = input[index].get(&map_indice) {
                if values.contains(&winning.parse::<u32>().unwrap()) {
                    if card_total == 0 {
                        card_total = 1;
                        return;
                    }
                    card_total *= 2;
                }
            }
        });
        total += card_total;
    });
    total
}

fn part2(input: Vec<HashMap<u32, Vec<u32>>>, winning: Vec<Vec<String>>) -> u32 {
    let mut card_repeat: Vec<u32> = vec![1;SIZE];
    winning.iter().enumerate().for_each(|(index, card)| {
        let mut card_wins = 0;
        card.iter().for_each(|winning| {
            let mut map_indice:u32 = 0;
            if winning.len() == 2 {
                map_indice = winning.chars().next().unwrap().to_digit(10).unwrap();
            }
            if let Some(values) = input[index].get(&map_indice) {
                if values.contains(&winning.parse::<u32>().unwrap()) {
                    card_wins += 1;
                }
            }
        });
        let total_by_x = card_repeat[index];
        card_repeat.iter_mut().skip(index+1).take(card_wins).for_each(|n| *n += total_by_x)
    });
    card_repeat.iter().sum()
}

fn main() {
    let (winning, input) = parse_input();
    // println!("Part 1 = {}", part1(input, winning));
    println!("Part 2 = {}", part2(input, winning));
}


