use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

#[derive(Debug)]
struct ScratchCard {
    my_numbers: HashSet<u32>,
    winning_number: HashSet<u32>,
}

impl ScratchCard {
    fn winning_count(&self) -> u32 {
        println!(
            "winnings {}",
            self.winning_number.intersection(&self.my_numbers).count() as u32
        );
        self.winning_number.intersection(&self.my_numbers).count() as u32
    }
}

#[derive(Debug)]
struct Game {
    game_id: u32,
    scratch_card: ScratchCard,
    copies: u32,
}

// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
fn main() {
    let lines = read_lines("src/bin/input.txt");

    let mut result = 0;
    let game_separator_index = lines[0].find(':').expect("There should be a ':' separator");
    let games: Vec<Game> = lines
        .iter()
        .map(|line| line.split_at(game_separator_index))
        .map(|(game_id_str, card_str)| {
            let game_id = game_id_str
                .split_whitespace()
                .last()
                .expect("The game should hava  last element")
                .parse::<u32>()
                .expect("the last element shuld be a number");
            let (mine_str, card_str) =
                card_str.split_at(card_str.find('|').expect("There should be a '|' separator"));

            let numbers = mine_str[1..].split_whitespace();
            let my_numbers = numbers
                .map(|num| num.parse::<u32>().expect("Should be able to convert it"))
                .collect();
            let numbers = card_str[1..].split_whitespace(); //skip '|'
            let winning_number = numbers
                .map(|number| number.parse::<u32>().expect("Should be able to convert it"))
                .collect();
            Game {
                game_id,
                scratch_card: ScratchCard {
                    my_numbers,
                    winning_number,
                },
                copies: 1,
            }
        })
        .collect();

    let mut scratch_count: HashMap<_, _> = games
        .iter()
        .map(|game| (game.game_id, game.copies))
        .collect();

    // dbg!(&scratch_count);
    for game in games {
        println!("{}", game.game_id);
        for idx in 0..game.scratch_card.winning_count() {
            let index = 1 + game.game_id + idx;
            println!("{}", idx);
            scratch_count.insert(index, scratch_count[&index] + scratch_count[&game.game_id]);
            dbg!(&scratch_count);
        }
        result += scratch_count
            .get(&game.game_id)
            .expect("every game should be mapped");
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
