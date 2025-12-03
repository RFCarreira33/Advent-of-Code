const MAX_DIAL: i32 = 100;

fn parse_input() -> Vec<(i32, i32)> {
    let mut intructions: Vec<(i32, i32)> = Vec::new();

    std::fs::read_to_string("input.txt")
        .expect("Error reading file")
        .split_whitespace()
        .for_each(|line| {
            let (action, value) = line.split_at(1);
            let action_bool = match action {
                "L" => -1,
                "R" => 1,
                _ => panic!("Invalid action"),
            };
            let value_int: i32 = value.parse().expect("Invalid value");
            intructions.push((action_bool, value_int));
        });

    intructions
}

fn part1(input: Vec<(i32, i32)>) -> i32 {
    let mut total: i32 = 0;
    let mut dial: i32 = 50;

    input.iter().for_each(|(action, value)| {
        dial = (dial + action * value) % MAX_DIAL;

        if dial == 0 {
            total += 1;
        }
    });

    total
}

fn part2(input: Vec<(i32, i32)>) -> i32 {
    let mut total = 0;
    let mut dial: i32 = 50;
    let mut dest_dial: i32 = 50;

    input.iter().for_each(|(action, value)| {
        let raw_dest_dial = dial + action * value;
        // dest_dial = raw_dest_dial % MAX_DIAL;
        // dest_dial = if dest_dial < 0 {
        //     dest_dial + MAX_DIAL
        // } else {
        //     dest_dial
        // };
        dest_dial = raw_dest_dial.rem_euclid(MAX_DIAL);

        let mut increment = (raw_dest_dial / MAX_DIAL).abs();
        if dest_dial == 0 && increment == 0 {
            increment += 1;
        }

        if raw_dest_dial < 0 && dial != 0 {
            increment += 1;
        }

        total += increment;
        dial = dest_dial;
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
