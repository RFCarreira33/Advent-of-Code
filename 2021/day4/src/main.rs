use std::{
    collections::{HashSet, VecDeque},
    str::Lines,
    vec,
};

struct Board {
    winning_row: Vec<HashSet<i32>>,
    completed: bool,
}

impl Board {
    // create new board and return it or return none to stop
    fn new(input: &mut Lines) -> Option<Board> {
        let mut winning_row: Vec<HashSet<i32>> = vec![];

        //remove empty line if next returns None input has ended
        if input.next() == None {
            return None;
        };

        let rows: Vec<Vec<i32>> = input
            .take(5)
            .map(|line| {
                line.split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect()
            })
            .collect();

        for col in 0..5 {
            let mut set = HashSet::new();
            for row in 0..5 {
                set.insert(rows[row][col]);
            }
            winning_row.push(set);
        }

        for row in rows {
            let mut set = HashSet::new();
            for num in row {
                set.insert(num);
            }
            winning_row.push(set);
        }

        // return Some if new board was created and None if not
        Some(Board {
            winning_row,
            completed: false,
        })
    }

    // check if any winning row is empty
    fn turn(&mut self, number: i32) -> bool {
        let mut win: bool = false;
        for set in self.winning_row.iter_mut() {
            if set.remove(&number) {
                if set.is_empty() {
                    // cant return early because the number most likely is in other set
                    win = true;
                }
            }
        }
        win
    }

    // return the sum of all unmarked numbers
    fn calc_score(&self) -> i32 {
        let mut no_dups: Vec<i32> = vec![];
        self.winning_row.iter().for_each(|set| {
            set.iter().for_each(|n| {
                //remove duplicates
                if !no_dups.contains(n) {
                    no_dups.push(*n);
                }
            })
        });
        no_dups.iter().sum()
    }
}

fn parse_input() -> (VecDeque<i32>, Vec<Board>) {
    // diferent method from past days but its an upgrade
    let mut input = include_str!("../input.txt").lines();

    let queue: VecDeque<i32> = input
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut boards: Vec<Board> = vec![];

    while let Some(board) = Board::new(&mut input) {
        boards.push(board);
    }

    return (queue, boards);
}

fn part1(mut queue: VecDeque<i32>, mut boards: Vec<Board>) -> i32 {
    while let Some(number) = queue.pop_front() {
        for board in &mut boards.iter_mut() {
            if board.turn(number) {
                // if the board is complete return the score
                return board.calc_score() * number;
            }
        }
    }
    // if no board is complete return 0
    return 0;
}

fn part2(mut queue: VecDeque<i32>, mut boards: Vec<Board>) -> i32 {
    let mut score: i32 = 0;
    let mut completed = 0;
    while let Some(number) = queue.pop_front() {
        //when the number of completed boards is equal to the number of boards
        //we can stop
        if completed == boards.len() {
            break;
        }
        for board in &mut boards.iter_mut() {
            // small optimization
            if board.completed {
                continue;
            }
            // if a board wins we mark it as completed
            if board.turn(number) {
                score = board.calc_score() * number;
                board.completed = true;
                completed += 1;
            }
        }
    }
    return score;
}

fn main() {
    let (queue, boards) = parse_input();
    // println!("Part 1 = {}\n", part1(queue, boards));
    println!("Part 2 = {}\n", part2(queue, boards));
}
