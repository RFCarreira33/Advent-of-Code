fn parse_input() -> Vec<Vec<u32>> {
    let mut banks: Vec<Vec<u32>> = Vec::new();

    std::fs::read_to_string("input.txt")
        .expect("Error reading file")
        .split_whitespace()
        .for_each(|line| {
            banks.push(
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>(),
            )
        });

    banks
}

fn part1(input: Vec<Vec<u32>>) -> u32 {
    let mut total: u32 = 0;

    input.iter().for_each(|bank| {
        let len = bank.len() - 1;
        let (max_index, max_value) = bank
            .iter()
            .rev()
            .skip(1)
            .enumerate()
            .max_by_key(|&(_, value)| value)
            .unwrap();

        total += 10u32.pow(1) * max_value;
        total += 10u32.pow(0) * bank.iter().skip(len - max_index).max().unwrap_or(&0);
    });

    total
}

fn part2(input: Vec<Vec<u32>>) -> u64 {
    let mut total: u64 = 0;

    input.iter().for_each(|bank| {
        let mut jolt: u64 = 0;
        let mut max_index = bank.len() - 11;

        (0..=11).rev().for_each(|i| {
            let (temp_max_index, max_value) = bank
                .iter()
                .rev()
                .skip(i)
                .take(max_index)
                .enumerate()
                .max_by_key(|&(_, value)| value)
                .unwrap();

            max_index = temp_max_index + 1;
            jolt += 10u64.pow(i as u32) * *max_value as u64;
        });

        total += jolt;
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
