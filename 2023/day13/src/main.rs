use std::usize;

type Matrix = Vec<Vec<char>>;

fn parse_input() -> Vec<Matrix> {
    let mut notes: Vec<Matrix> = Vec::new();
    let mut matrix: Matrix = Vec::new();
    std::fs::read_to_string("input.txt")
        .expect("Error reading file")
        .split_terminator("\n")
        .for_each(|line| {
            if line.len() > 0 {
                matrix.push(line.chars().collect::<Vec<char>>());
                return;
            }
            notes.push(matrix.to_owned());
            matrix = Vec::new();
        });
    notes.push(matrix);
    notes
}

fn rotate(matrice: &Matrix) -> Matrix {
    let mut new_matrice: Matrix = Vec::new();
    (0..matrice[0].len()).for_each(|i| {
        let mut line: Vec<char> = Vec::new();
        ((0..matrice.len()).rev()).for_each(|j| {
            line.push(matrice[j][i]);
        });
        new_matrice.push(line);
    });
    new_matrice
}

fn find(matrix: &Matrix, smudging: bool) -> usize {
    let mut prev_row: &Vec<char> = &matrix[0];
    for (i, row) in matrix.iter().skip(1).enumerate() {
        let off_by = row.iter().zip(prev_row).fold(0, |count, (a,b)|{
            if count > 1 {
                return count;
            }
            if a != b {
                return count + 1
            };
            count
        });
        if !smudging && off_by == 0 && check_posibility(&matrix, i, None) {
            return i+1;
        }
        if smudging && off_by <= 1 && check_posibility(&matrix, i, Some(off_by)) {
            return i+1;
        }
        prev_row = row;
    }
    0
}

fn check_posibility(matrix: &Matrix, index: usize, smudges: Option<i32>) -> bool {
    let min = (matrix.len()-1 - index).min(index);
    let mut off_by = smudges.unwrap_or(0);
    for i in (0..min).into_iter() {
        if index+i+2 > matrix.len()-1 {
            break;
        }

        off_by = matrix[index-1-i].iter().zip(&matrix[index+2+i]).fold(off_by, |count, (a,b)|{
            if count > 1 {
                return count;
            }
            if a != b {
                return count + 1
            };
            count
        });
        
        if smudges.is_none() && off_by != 0 {
            return false;
        }
    }
    if smudges.is_some() && off_by != 1 {
        return false;
    }
    true
}

#[allow(dead_code)]
fn part1(input: Vec<Matrix>) -> usize {
    let mut total = 0;
    input.iter().for_each(|matrix|{
        total += find(&rotate(matrix), false) + find(matrix, false) *100;
    });
    total
}

fn part2(input: Vec<Matrix>) -> usize {
    let mut total = 0;
    input.iter().for_each(|matrix|{
        total += find(&rotate(matrix), true) + find(matrix, true) *100;
    });
    total
}

fn main() {
    let input = parse_input();
    // println!("Part 1 = {}", part1(input));
    println!("Part 2 = {}", part2(input));
}




