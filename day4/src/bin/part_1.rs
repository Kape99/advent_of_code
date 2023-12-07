use std::fs::read_to_string;

// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11

fn main() {
    let lines = read_lines("src/bin/input.txt");
    let game_separator_index = lines[0].find(':').expect("There should be a ':' separator");
    let lines: Vec<(Vec<u32>, Vec<u32>)> = lines
        .iter()
        .map(|line| line.split_at(game_separator_index))
        .map(|(_, game)| game)
        .map(|game| game[1..].split_at(game.find('|').expect("There should be a '|' separator")))
        .map(|(mine_str, card_str)| {
            let numbers = mine_str.split_whitespace();
            let my_numbers = numbers
                .map(|number| number.parse::<u32>().expect("Should be able to convert it"))
                .collect();
            let numbers = card_str[1..].split_whitespace(); //skip '|'
            let card_numbers = numbers
                .map(|number| number.parse::<u32>().expect("Should be able to convert it"))
                .collect();
            (my_numbers, card_numbers)
        })
        .collect();

    let mut result = 0;

    let base: i32 = 2;
    for line in lines {
        let mut exp: u32 = 0;
        for my in line.0 {
            if line.1.contains(&my) {
                exp += 1
            }
        }
        if exp > 0 {
            result += base.pow(exp - 1)
        }
    }

    println!("{result}")
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
