use std::{char, fs::read_to_string, usize};

fn main() {
    let lines = read_lines("src/bin/input.txt");

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let line = &line;
        let mut list: Vec<char> = Vec::new();
        for col in 0..line.len() {
            list.push(line.chars().nth(col).unwrap());
        }
        matrix.push(list)
    }

    let mut total = 0;

    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            if let Some(number) = matrix[row][col].to_digit(10) {
                if col != 0 && matrix[row][col - 1].is_ascii_digit() {
                    continue;
                }

                let mut value: u64 = number as u64;
                let mut is_valid = is_valid_cell(matrix.clone(), (row as i32, col as i32));

                let mut next_digit = col + 1.clamp(0, matrix[row].len());

                while let Some(new_unit) = matrix[row][next_digit].to_digit(10) {
                    value *= 10;
                    value += new_unit as u64;
                    println!("{value}");
                    if is_valid_cell(matrix.clone(), (row as i32, next_digit as i32)) {
                        is_valid = true
                    }

                    next_digit += 1;
                    if next_digit == matrix[row].len() {
                        break;
                    }
                }

                if is_valid {
                    total += value;
                }
            }
        }
    }
    println!("{total}")
}

fn is_valid_cell(matrix: Vec<Vec<char>>, (row, col): (i32, i32)) -> bool {
    for r in -1..2 {
        for c in -1..2 {
            let ro: usize = (row + r).clamp(0, matrix.len() as i32 - 1) as usize;
            let co = (col + c).clamp(0, matrix[ro].len() as i32 - 1);

            if matrix[ro][co as usize].to_digit(10).is_none() && '.' != matrix[ro][co as usize] {
                return true;
            }
        }
    }
    false
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    match read_to_string(filename) {
        Ok(lines) => {
            for line in lines.lines() {
                result.push(line.to_string());
            }
        }
        Err(msg) => println!("ERROR: {msg}"),
    }
    result
}
