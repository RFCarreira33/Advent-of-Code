fn parse_input() -> (Vec<Vec<u32>>, Vec<char>) {
    let mut numbers: Vec<Vec<u32>> = Vec::new();
    let mut operators: Vec<char> = Vec::new();

    std::fs::read_to_string("input.txt")
        .expect("Error reading file")
        .split_terminator('\n')
        .for_each(|line| {
            line.split_whitespace().enumerate().for_each(|(i, token )| {
                if let Ok(num) = token.parse::<u32>() {
                    if numbers.len() <= i {
                        numbers.push(vec![num]);
                        return;
                    }

                    numbers[i].push(num);
                    return;
                }

                if let Some(op) = token.chars().next() {
                    operators.push(op);
                }
            });

        });

    (numbers, operators)
}

fn part1(number_groups: Vec<Vec<u32>>, operations: Vec<char>) -> u64 {
    let mut total: u64 = 0;

    number_groups.iter().enumerate().for_each(|(i, group)| {
        let mult = operations.get(i).unwrap_or(&'*') == &'*';
        let mut group_total: u64 = group[0] as u64;

        group.iter().skip(1).for_each(|num| {
            if mult {
                group_total *= *num as u64;
                return;
            }

            group_total += *num as u64;
        });

        total += group_total;
    });

    total
}

fn part2(number_groups: Vec<Vec<u32>>, operations: Vec<char>) -> u64 {
    let mut total: u64 = 0;

    total
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Please provide an argument for which part to run");
        return;
    }

    let (numbers, ops) = parse_input();
    match args[1].as_str() {
        "1" => {
            println!("Part 1: {}", part1(numbers, ops));
        }
        "2" => {
            println!("Part 2: {}", part2(numbers, ops));
        }
        _ => println!("Invalid argument. Please use 1 or 2"),
    }
}
