fn parse_input() -> (Vec<u32>,Vec<u32>) {
    let mut times : Vec<u32> = vec![];
    let mut result : Vec<u32> = vec![];
    std::fs::read_to_string("input.txt")
        .expect("Error reading file")
        .split_terminator("\n")
        .enumerate()
        .for_each(|(line_number,line)| line.split_whitespace().skip(1).for_each(|num| {
            let number = num.parse::<u32>().unwrap();
            if line_number == 0 {
                times.push(number);
                return;
            }   
            result.push(number);
        }));
    (times, result)
}

#[allow(dead_code)]
fn part1(times:Vec<u32>, results:Vec<u32>) -> u32 {
    let mut counter = 1;
    times.iter().zip(results.iter()).for_each(|(t, r)| {
        for (t1, t2) in (0..*t).zip((0..=*t).rev()) {
            if t1 * t2 > *r {
                counter *= t2 - t1 + 1;
                break;
            }
        }
    });
    counter
}

fn part2(times:Vec<u32>, results:Vec<u32>) -> u64 {
    let big_time: String = times.iter().map(|n| n.to_string()).collect();
    let big_result: String = results.iter().map(|n| n.to_string()).collect();
    let time = big_time.parse::<u64>().unwrap();
    let result = big_result.parse::<u64>().unwrap();
    for (t1, t2) in (0..time).zip((0..=time).rev()) {
        if t1 * t2 > result {
            return t2 - t1 + 1;
        }
    }
    0
}

fn main() {
    let (times, results) = parse_input();
    // println!("Part 1 = {}", part1(times,results));
    println!("Part 2 = {}", part2(times,results));
}

