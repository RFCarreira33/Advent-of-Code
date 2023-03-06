use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq)]
struct Coordenate {
    x: i32,
    y: i32,
}

fn parse_input() -> Vec<Coordenate> {
    let mut input = include_str!("../input.txt").lines();

    let mut coords: Vec<Coordenate> = Vec::new();
    while let Some(line) = input.next() {
        let line: Vec<String> = line
            .trim()
            .split_terminator(" -> ")
            .map(|s| s.to_string())
            .collect();

        for s in line {
            let coord: Vec<i32> = s
                .trim()
                .split_terminator(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect();

            coords.push(Coordenate {
                x: coord[0],
                y: coord[1],
            });
        }
    }
    return coords;
}

#[allow(dead_code)]
fn part1(input: Vec<Coordenate>) -> usize {
    let mut coords: HashSet<Coordenate> = HashSet::new();
    let mut duplicates: HashSet<Coordenate> = HashSet::new();
    // iterate 2 by 2
    for pair in input.windows(2).step_by(2) {
        // create a copy of the first coordenate
        let mut begin = Coordenate {
            x: pair[0].x,
            y: pair[0].y,
        };
        // verifie if not a diagonal line
        if begin.x == pair[1].x || begin.y == pair[1].y {
            // loop until the x's match
            while begin.x != pair[1].x {
                if !coords.insert(Coordenate {
                    x: begin.x,
                    y: begin.y,
                }) {
                    duplicates.insert(Coordenate {
                        x: begin.x,
                        y: begin.y,
                    });
                };
                begin.x += (pair[1].x - begin.x).signum();
            }
            // loop until the y's match
            while begin.y != pair[1].y {
                if !coords.insert(Coordenate {
                    x: begin.x,
                    y: begin.y,
                }) {
                    duplicates.insert(Coordenate {
                        x: begin.x,
                        y: begin.y,
                    });
                };
                begin.y += (pair[1].y - begin.y).signum();
            }
            // add last coordenate
            if !coords.insert(Coordenate {
                x: pair[1].x,
                y: pair[1].y,
            }) {
                duplicates.insert(Coordenate {
                    x: pair[1].x,
                    y: pair[1].y,
                });
            }
        }
    }
    return duplicates.len();
}

fn part2(input: Vec<Coordenate>) -> usize {
    let mut coords: HashSet<Coordenate> = HashSet::new();
    let mut duplicates: HashSet<Coordenate> = HashSet::new();
    // iterate 2 by 2
    for pair in input.windows(2).step_by(2) {
        // create a copy of the first coordenate
        let mut begin = Coordenate {
            x: pair[0].x,
            y: pair[0].y,
        };
        // loop until the coordenates match
        while begin.x != pair[1].x || begin.y != pair[1].y {
            if !coords.insert(Coordenate {
                x: begin.x,
                y: begin.y,
            }) {
                duplicates.insert(Coordenate {
                    x: begin.x,
                    y: begin.y,
                });
            };
            if begin.x != pair[1].x {
                begin.x += (pair[1].x - begin.x).signum();
            }
            if begin.y != pair[1].y {
                begin.y += (pair[1].y - begin.y).signum();
            }
        }
        // insert the last coordenate
        if !coords.insert(Coordenate {
            x: pair[1].x,
            y: pair[1].y,
        }) {
            duplicates.insert(Coordenate {
                x: pair[1].x,
                y: pair[1].y,
            });
        }
    }
    return duplicates.len();
}

fn main() {
    let input = parse_input();
    // println!("Part 1 = {}\n", part1(input));
    println!("Part 2 = {}\n", part2(input));
}
