fn parse_input() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

fn scuffedf(str: &str) -> i32 {
    let mut total: i32 = 0;

    str.split("mul(").for_each(|slice| {
        slice.split(")").for_each(|s| {
            let numbers = s.split(",").collect::<Vec<&str>>();
            if numbers.len() != 2 {
                return;
            }

            let mut nums: Vec<i32> = Vec::new();
            numbers.iter().for_each(|n| {
                if let Ok(num) = n.parse::<i32>() {
                    nums.push(num);
                }
            });

            if nums.len() != 2 {
                return;
            }

            total += nums[0] * nums[1];
        })
    });

    total
}

fn part1(input: String) -> i32 {
    scuffedf(&input)
}

fn part2(input: String) -> i32 {
    let mut total: i32 = 0;

    input.split("do()").for_each(|slice| {
        let parts = slice.split("don't()").collect::<Vec<&str>>();
        total += scuffedf(parts[0]);
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
