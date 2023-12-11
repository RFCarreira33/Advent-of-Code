const MULTIPLIER: i32 = 999999;

fn rotate(matrice: Vec<Vec<bool>>, reverse:bool) -> Vec<Vec<bool>> {
    let mut new_matrice: Vec<Vec<bool>> = Vec::new();
    if reverse {
        ((0..matrice[0].len()).rev()).for_each(|i| {
            let mut line: Vec<bool> = Vec::new();
            (0..matrice.len()).for_each(|j| {
                line.push(matrice[j][i]);
            });
            new_matrice.push(line);
        });
        return new_matrice;
    }
    (0..matrice[0].len()).for_each(|i| {
        let mut line: Vec<bool> = Vec::new();
        ((0..matrice.len()).rev()).for_each(|j| {
            line.push(matrice[j][i]);
        });
        new_matrice.push(line);
    });
    new_matrice
}

fn parse_input() -> (Vec<(i32, i32)>, Vec<i32>, Vec<i32>) {
    let mut matrice: Vec<Vec<bool>> = Vec::new();
    let mut padding_x: Vec<i32> = Vec::new();
    let mut padding_y: Vec<i32> = Vec::new();
    std::fs::read_to_string("input.txt")
        .expect("Error reading file")
        .split_terminator("\n")
        .enumerate()
        .for_each(|(li,line)| {
            let mut line_vec: Vec<bool> = Vec::new();
            line.chars().for_each(|c| {
                if c == '#' {
                    line_vec.push(true);
                    return;
                }
                line_vec.push(false);
            });
            if line_vec.iter().any(|&x| x) {
                matrice.push(line_vec);
                return;
            }
            padding_x.push(li as i32);
            matrice.push(line_vec);
        });
    matrice = rotate(matrice, false);
    matrice.iter().enumerate().for_each(|(i, line)| {
        if line.iter().any(|&x| x) {
            return;
        }
        padding_y.push(i as i32);
    });
    matrice = rotate(matrice, true);
    let mut galaxies: Vec<(i32, i32)> = Vec::new();
    matrice.iter().enumerate().for_each(|(li, line)| {
        line.iter().enumerate().for_each(|(ci, c)| {
            if *c {
                galaxies.push((li as i32, ci as i32));
            }
        });
    });
    (galaxies, padding_x, padding_y)
}

#[allow(dead_code)]
fn part1(galaxies: Vec<(i32, i32)>, padding_x: Vec<i32>, padding_y: Vec<i32>) -> i32 {
    let mut steps: i32 = 0;
    galaxies.clone().iter().enumerate().for_each(|(gi, galaxy1)| {
        galaxies.iter().skip(gi).for_each(|galaxy2| {
            let x_count = padding_x.iter().filter(|&&x| x > galaxy1.0 && x < galaxy2.0 || x > galaxy2.0 && x < galaxy1.0).count();
            let y_count = padding_y.iter().filter(|&&y| y > galaxy1.1 && y < galaxy2.1 || y > galaxy2.1 && y < galaxy1.1).count();
            steps += (galaxy1.0 - galaxy2.0).abs() + (galaxy1.1 - galaxy2.1).abs() + (x_count + y_count) as i32;
        });
    });
    steps
}

fn part2(galaxies: Vec<(i32, i32)>, padding_x: Vec<i32>, padding_y: Vec<i32>) -> i64 {
    let mut steps: i64 = 0;
    galaxies.clone().iter().enumerate().for_each(|(gi, galaxy1)| {
        galaxies.iter().skip(gi).for_each(|galaxy2| {
            let x_count = padding_x.iter().filter(|&&x| x > galaxy1.0 && x < galaxy2.0 || x > galaxy2.0 && x < galaxy1.0).count();
            let y_count = padding_y.iter().filter(|&&y| y > galaxy1.1 && y < galaxy2.1 || y > galaxy2.1 && y < galaxy1.1).count();
            steps += ((galaxy1.0 - galaxy2.0).abs() + (galaxy1.1 - galaxy2.1).abs() + (x_count as i32*MULTIPLIER) + (y_count as i32*MULTIPLIER)) as i64;
        });
    });
    steps
}

fn main() {
    let (input, x, y) = parse_input();
    // println!("Part 1 = {}", part1(input, x, y));
    println!("Part 2 = {}", part2(input, x, y));
}




