use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let games = read_lines("input.txt");
    let mut game_sum = 0;
    for game in &games {
        let (game_id, round_str) = get_id_and_rounds(game);
        
        let is_valid = is_valid_game(&round_str);

        match is_valid {
            true => game_sum += game_id,
            _ => {}
        }

    }
    println!("Game sum: {game_sum}");
}

fn is_valid_game(round_str: &str) -> bool {
    let round_list_str: Vec<_> = round_str[1..].split(";").collect();
    
    for round in round_list_str {
        if !is_valid_round(round){
            return false;
        }
    }

    true
}

fn is_valid_round(round: &str) -> bool{
    let bag = HashMap::from([(" red", 12), (" green", 13), (" blue", 14)]);

    let cubes_str: Vec<_> = round[1..].split(", ").collect();

    for cube_str in cubes_str {
        let (color, number) = match cube_str.find(" ") {
            Some(split_index) => {
                let (n, c) = cube_str.split_at(split_index);
                let num = match n.parse::<i32>() {
                    Ok(id) => id,
                    Err(msg) => panic!("Error: {msg}"),
                };

                (c, num)
            }
            None => panic!("the cube divisor was not found"),
        };

        match bag.get(color) {
            Some(pool) => {
                if pool < &number {
                    return false;
                }
            }
            _ => {}
        }
    }
    true
}

fn get_id_and_rounds(game: &str) -> (u32, String) {
    let (game_id_str, rounds_str) = match game.find(":") {
        Some(split_index) => game.split_at(split_index),
        None => panic!("the game divisor was not found"),
    };

    let game_id = match game_id_str[5..].parse::<u32>() {
        Ok(id) => id,
        Err(msg) => panic!("Error: {msg}"),
    };
    
    (game_id, rounds_str.to_owned())
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
