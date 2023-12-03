use std::{fs::read_to_string, collections::hash_set};

fn main() {
    let lines = read_lines("src/bin/input.txt");

    let mut matrix: Vec<Vec<char>> = Vec::new();

    let mut computed_result = 0;

    for line in lines {
        let line = &line;
        let mut list: Vec<char> = Vec::new();
        for col in 0..line.len() {
            list.push(line.chars().nth(col).unwrap());
        }
        matrix.push(list)
    }




    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            if '*' == matrix[row][col] {
                compute_mult(&matrix, row, col);

            }
        }
    }

    println!("{computed_result}")
}

fn compute_mult( matrix: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    let start_r = if row == 0 { 0 } else { row - 1};
    let end_r = if row == matrix.len() { matrix.len() } else { row + 1};

    let start_c = if col == 0 { 0 } else { col - 1};
    let end_c = if col == matrix[row].len() { matrix[row].len() } else { col + 1};

    let mut numbers: Vec<u32> = Vec::new();
    for n_row in start_r..end_r+1{
        for n_col in start_c..end_c+1{
            if 0 != get_number(matrix.clone(), (n_row, n_col)){
                if n_col == start_c ||  0 == get_number(matrix.clone(), (n_row, n_col-1)){

                
                    numbers.push(get_number(matrix.clone(), (n_row, n_col)));
                }
            
            }
        
        }
    }
    let mut mult = 0;
    if numbers.len() == 2{
        mult = 1;
        for n in numbers{
            println!("{n}");
            mult*= n;
        }
    }
    return mult;
}

fn get_number(matrix: Vec<Vec<char>>, (row, col): (usize, usize)) -> u32 {
    if !matrix[row][col].is_ascii_digit(){
        return 0;
    }

    let mut index = col;
    let mut value = 0;
    loop {
        if matrix[row][index].is_ascii_digit() {
            if index == 0 {
                break;
            }
            index -= 1;
 
        }else{
            index += 1;
            break;
        }
        
    }

    loop {
        if let Some(unit) = matrix[row][index].to_digit(10) {
            value *= 10;
            value += unit;
            if index == matrix[row].len() {
                break;
            }
            index += 1;
            if index == matrix[row].len(){
                break;
            }
        } else {
            break;
        }
    }
    return value;
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
