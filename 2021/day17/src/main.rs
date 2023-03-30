const HELPER: [(i32, i32); 2] = [(-1, -1), (0, -1)];

fn parse_input() -> ((i32, i32), (i32, i32)) {
    let input = include_str!("../input.txt").to_string();
    let (x, y) = input
        .trim()
        .trim_start_matches("target area: ")
        .split_once(", ")
        .unwrap();

    let (x_s, x_e) = x.trim_start_matches("x=").split_once("..").unwrap();
    let (y_s, y_e) = y.trim_start_matches("y=").split_once("..").unwrap();

    (
        (x_s.parse().unwrap(), x_e.parse().unwrap()),
        (y_s.parse().unwrap(), y_e.parse().unwrap()),
    )
}

fn try_speed(x_s: &(i32, i32), y_s: &(i32, i32), mut speed: (i32, i32)) -> Option<i32> {
    let mut current: (i32, i32) = (0, 0);
    let mut highest: i32 = 0;
    let mut index = 0;
    loop {
        // update current, speed and highest point
        current = (current.0 + speed.0, current.1 + speed.1);
        speed = (speed.0 + HELPER[index].0, speed.1 + HELPER[index].1);
        if speed.0 == 0 {
            index = 1;
        }
        highest = highest.max(current.1);

        //check if its the target
        if current.0 >= x_s.0 && current.0 <= x_s.1 && current.1 >= y_s.0 && current.1 <= y_s.1 {
            return Some(highest);
        }

        // we have gone too far
        if current.0 > x_s.1 && current.1 > y_s.1 {
            return None;
        }

        // if there is no more x movement and we are not on top of the target
        if speed.0 == 0 && !(current.0 >= x_s.0 && current.0 <= x_s.1) {
            return None;
        }

        // if theres is no y speed and were below the target skip
        if speed.1 < 0 && current.1 < y_s.0 {
            return None;
        }
    }
}

#[allow(dead_code)]
fn part1(x_s: (i32, i32), y_s: (i32, i32)) -> i32 {
    let mut highest = 0;
    for x in -215..=215 {
        for y in -215..=215 {
            if let Some(h) = try_speed(&x_s, &y_s, (x, y)) {
                highest = highest.max(h);
            }
        }
    }
    highest
}

fn part2(x_s: (i32, i32), y_s: (i32, i32)) -> i32 {
    let mut count = 0;
    for x in -215..=215 {
        for y in -215..=215 {
            if try_speed(&x_s, &y_s, (x, y)).is_some() {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let (x_s, y_s) = parse_input();
    // println!("Part 1 = {}", part1(x_s, y_s));
    println!("Part 2 = {}", part2(x_s, y_s));
}
