use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let games = read_lines("input.txt");
    
    let mut count = 0;
    for game in &games {
        let (game_id, round_str) = get_id_and_rounds(game);
        
        let round_list_str: Vec<_> = round_str[1..].split(";").collect();
    
        let mut bag = HashMap::from([(" red", 0), (" green", 0), (" blue", 0)]);

        for round in round_list_str {

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
                if bag.get(color).unwrap() < &number{
                    *bag.get_mut(color).unwrap() = number;
                }
            }
        }
        let mut exp = 1;
        for val in bag.values(){
            exp *= val;

        }
        count += exp;
        
    }

    println!("{count}");
    // let game_sum = compute_valid_games(games);
    // println!("Game sum: {game_sum}");
}

fn compute_valid_games(games: Vec<String>) -> u32{
    let mut game_sum = 0;
    for game in &games {
        let (game_id, round_str) = get_id_and_rounds(game);
    
        let is_valid = is_valid_game(&round_str);

        match is_valid {
            true => game_sum += game_id,
            _ => {}
        }

    }
    game_sum
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
