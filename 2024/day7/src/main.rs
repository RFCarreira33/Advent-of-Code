fn parse_input() -> (Vec<Vec<u64>>, Vec<u64>) {
    let mut numbers: Vec<Vec<u64>> = Vec::new();
    let mut results: Vec<u64> = Vec::new();

    std::fs::read_to_string("input.txt")
        .expect("Error reading file")
        .split_terminator("\n")
        .for_each(|line| {
            let parts = line.split(":").collect::<Vec<&str>>();
            results.push(parts[0].parse().unwrap());

            let numbers_line = parts[1].split_whitespace().collect::<Vec<&str>>();
            numbers.push(numbers_line.iter().map(|x| x.parse().unwrap()).collect());
        });

    ( numbers, results)
}

fn is_valid(numbers: Vec<u64>, cur:u64, result: u64, p2: bool) -> bool {
    if numbers.is_empty() {
        return result == cur;
    }

    let f = numbers[0];
    let mut operations = vec![cur + f, cur * f];

    if p2 {
        operations.push(concat(cur, f));
    }

    if operations.iter().all(|x| *x > result) {
        return false;
    }

    if operations.iter().any(|x| is_valid(numbers[1..].to_vec(), *x, result, p2)) {
        return true;
    }

    false
}

// insert folded hand emoji https://www.reddit.com/r/rust/comments/191l3ot/concatinate_two_numbers/
fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}

fn part1(numbers: Vec<Vec<u64>>, results: Vec<u64>) -> u64 {
    let mut total = 0;

    numbers.iter().enumerate().for_each(|(i, numbers)| {
        if is_valid(numbers.to_vec(), 0, results[i], false) {
            total += results[i];
        }
    });

    total
}

fn part2(numbers: Vec<Vec<u64>>, results: Vec<u64>) -> u64 {
    let mut total = 0;

    numbers.iter().enumerate().for_each(|(i, numbers)| {
        if is_valid(numbers.to_vec(), 0, results[i], true) {
            total += results[i];
        }
    });

    total
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Please provide an argument for which part to run");
        return;
    }

    let ( numbers, results ) = parse_input();
    match args[1].as_str() {
        "1" => {
            println!("Part 1: {}", part1(numbers, results));
        }
        "2" => {
            println!("Part 2: {}", part2(numbers, results));
        }
        _ => println!("Invalid argument. Please use 1 or 2"),
    }
}

