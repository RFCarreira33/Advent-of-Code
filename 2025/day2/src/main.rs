struct IdRange {
    start: i64,
    end: i64,
}

fn parse_input() -> Vec<IdRange> {
    let mut ids: Vec<IdRange> = Vec::new();

    std::fs::read_to_string("input.txt")
        .expect("Error reading file")
        .split(",")
        .for_each(|slice| {
            let bounds: Vec<i64> = slice
                .trim()
                .split("-")
                .map(|num_str| num_str.parse::<i64>().unwrap())
                .collect();

            ids.push(IdRange {
                start: bounds[0],
                end: bounds[1],
            });
        });

    ids
}

fn part1(input: Vec<IdRange>) -> i64 {
    let mut total: i64 = 0;

    input.iter().for_each(|id_range| {
        (id_range.start..=id_range.end).for_each(|id_val| {
            let id_str = id_val.to_string();
            let len = id_str.len();

            if len % 2 != 0 {
                return;
            }

            let half = len / 2;
            if id_str[half..] == id_str[..half] {
                return;
            }

            total += id_val;
        })
    });

    total
}

fn check_valid(id_str: &str, j: usize) -> bool {
    let pattern: Vec<char> = id_str.chars().take(j).collect();

    for window in id_str
        .chars()
        .skip(j)
        .collect::<Vec<char>>()
        .windows(j)
        .step_by(j)
    {
        if window != pattern {
            return false;
        }
    }

    true
}

fn part2(input: Vec<IdRange>) -> i64 {
    let mut total: i64 = 0;

    input.iter().for_each(|id_range| {
        (id_range.start..=id_range.end).for_each(|id_val| {
            let id_str = id_val.to_string();
            let len = id_str.len();

            for j in 1..=(len / 2) {
                if len % j != 0 {
                    continue;
                }

                if check_valid(&id_str, j) {
                    total += id_val;
                    break;
                }
            }
        })
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
