use std::{fs::read_to_string, collections::hash_set};


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

fn main() {
    let lines = read_lines("src/bin/input.txt");

}


