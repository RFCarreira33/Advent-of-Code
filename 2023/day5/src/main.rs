#[derive(Debug)]
struct Instructions {
    dest_start: i64,
    source_start: i64,
    source_end: i64,
}

impl Instructions {
    fn new(dest_range_start: i64, len: i64, source_range_start: i64) -> Instructions {
        Instructions {
            dest_start: dest_range_start,
            source_start: source_range_start,
            source_end: source_range_start + len,
        }
    }

    fn compare(&self, seed: &mut i64) -> bool {
        if *seed >= self.source_start && *seed <= self.source_end {
            let dif = self.dest_start - self.source_start;
            *seed += dif;
            return true;
        }
        false
    }
}

fn parse_input() -> (Vec<i64>, Vec<Vec<Instructions>>) {
    let mut instructions_paper: Vec<Vec<Instructions>> = vec![vec![]];
    let mut input = include_str!("../input.txt").lines();
    let mut index: i64 = -1;
    let seeds = input
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|seed| seed.parse::<i64>().unwrap())
        .collect();
    input.for_each(|line| {
        if line == "" {
            return;
        }
        if line.ends_with(':') {
            instructions_paper.push(vec![]);
            index += 1;
            return;
        }
        let mut ds: i64 = 0;
        let mut ss: i64 = 0;
        let mut len: i64 = 0;
        line.split_whitespace()
            .enumerate()
            .take(3)
            .for_each(|(i, number)| {
                let number = number.parse::<i64>().unwrap();
                match i {
                    0 => ds = number,
                    1 => ss = number,
                    2 => len = number,
                    _ => unreachable!(),
                }
            });
        instructions_paper[index as usize].push(Instructions::new(ds, len, ss))
    });

    (seeds, instructions_paper)
}

#[allow(dead_code)]
fn part1(input: Vec<Vec<Instructions>>, mut seeds: Vec<i64>) -> i64 {
    seeds.iter_mut().for_each(|seed| {
        input.iter().for_each(|instruction_vec| {
            instruction_vec
                .iter()
                .any(|instruction| instruction.compare(seed));
        });
    });
    *seeds.iter().min().unwrap()
}

fn part2(input: Vec<Vec<Instructions>>, seeds: Vec<i64>) -> i64 {
    let mut new_seeds: Vec<i64> = vec![];
    seeds.windows(2).step_by(2).into_iter().for_each(|window| {
        (window[0]..window[0] + window[1]).for_each(|seed| new_seeds.push(seed))
    });

    new_seeds.iter_mut().for_each(|seed| {
        input.iter().for_each(|instruction_vec| {
            instruction_vec
                .iter()
                .any(|instruction| instruction.compare(seed));
        });
    });
    *new_seeds.iter().min().unwrap() - 1
}

fn main() {
    let (seeds, input) = parse_input();
    // println!("Part 1 = {}", part1(input, seeds));
    println!("Part 2 = {}", part2(input, seeds));
}
