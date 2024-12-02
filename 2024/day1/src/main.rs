use std::collections::HashMap;

fn parse_input() -> (Vec<i32>, Vec<i32>) {
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();

    std::fs::read_to_string("input.txt")
        .expect("Error reading file")
        .split_whitespace()
        .enumerate()
        .for_each(|(i, num)| {
            if i % 2 == 0 {
                vec1.push(num.parse().unwrap());
            } else {
                vec2.push(num.parse().unwrap());
            }
        });

    return (vec1, vec2);
}

fn part1(input: (Vec<i32>, Vec<i32>)) -> i32 {
    let (mut vec1, mut vec2) = input;
    let mut total: i32 = 0;
    vec1.sort();
    vec2.sort();

    vec1.iter().zip(vec2.iter()).for_each(|(num1, num2)| {
        total += (num1 - num2).abs();
    });

    total
}

fn part2(input: (Vec<i32>, Vec<i32>)) -> i32 {
    let (mut vec1, mut vec2) = input;
    let mut total: i32 = 0;
    let mut last_index: usize = 0;
    let mut hash_map: HashMap<i32, i32> = HashMap::new();
    vec1.sort();
    vec2.sort();

    vec1.iter().for_each(|n1| {
        if hash_map.contains_key(n1) {
            total += hash_map[n1];
            return;
        }

        let mut counter = 0;

        for i in last_index..vec2.len() {
            if n1 == &vec2[i] {
                counter += 1;
                continue;
            }

            if n1 < &vec2[i] {
                last_index = i;
                break;
            }
        }

        let n_total = counter * n1;
        hash_map.insert(*n1, n_total);
        total += n_total;
    });

    total
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Please provide an argument for which part to run");
        return;
    }

    let input = parse_input();
    match args[1].as_str() {
        "1" => {
            println!("Part 1: {}", part1(input));
        }
        "2" => {
            println!("Part 2: {}", part2(input));
        }
        _ => println!("Invalid argument. Please use 1 or 2"),
    }
}
