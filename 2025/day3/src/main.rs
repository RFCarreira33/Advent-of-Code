
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

fn concat_digits(a: u32, b: u32) -> u32 {
    a / 10 * 10 + b
}

fn part1(input: Vec<Vec<u32>>) -> u32 {
    let mut total: u32 = 0;

    input.iter().for_each(|bank| {
        let mut highest: u32 = 0;
        let mut second_highest: u32 = 0;

        bank.iter().for_each(|bat| {
            let ten_x = *bat * 10;

            if ten_x > highest {
                second_highest = highest;
                highest = ten_x;
                return;
            } 

            if concat_digits(highest, *bat) > highest {
                second_highest = highest;
                highest = concat_digits(highest, *bat);
                return;
            }

            if ten_x > second_highest {
                second_highest = ten_x;
                return;
            }

            if concat_digits(second_highest, *bat) > second_highest {
                second_highest = concat_digits(second_highest, *bat);
                return;
            }
        });

        if highest % 10 == 0 {
            highest = second_highest / 10 * 10 + highest / 10;
        }
        total += highest
    });

    total
}

fn part2(input: Vec<Vec<u32>>) -> u64 {
    let mut total: u64 = 0;

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
