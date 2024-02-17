use std::{collections::HashMap, fs::read_to_string, ops::Index};

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
    println!("the result is: {}", compute("src/bin/input.txt"))
}

#[derive(Debug)]
struct Game {
    hand: String,
    bid: u64,
}

impl Game {
    fn value(&self) -> u8 {
        let mut point: HashMap<char, u8> = HashMap::new();
        for c in self.hand.chars() {
            match point.get_mut(&c) {
                Some(mut num) => {
                    *num += 1;
                }
                None => {
                    point.insert(c, 1);
                }
            }
        }
        match point.len() {
            1 => 10,
            2 => {
                if *point.iter().last().expect("").1 == 1 || *point.iter().last().expect("").1 == 4
                {
                    8
                } else {
                    6
                }
            }
            3 => {
                let mut res = 4;
                for p in point {
                    if p.1 == 3 {
                        res = 5
                    }
                }
                res
            }
            4 => 2,
            _ => 0,
        }
    }
}

// impl Ord for Game {
//     fn cmp(&self, other: &Self) -> Ordering {
//         (self.hand).cmp(&(&other.hand))
//     }
// }

// impl PartialOrd for Game {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        (&self.hand) == (&other.hand)
    }
}

impl Eq for Game {}

fn compute(path: &str) -> u64 {
    let lines: Vec<String> = read_lines(path);

    let mut games: Vec<Game> = lines
        .iter()
        .map(|line| {
            let game: Vec<&str> = line.split_whitespace().collect();
            Game {
                hand: game[0].to_string(),
                bid: game[1].parse::<u64>().expect("should be a number"),
            }
        })
        .collect();

    for g in games {
        println!("{:?}", g);
    }
    // games.sort();

    return 0;
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_compute() {
        assert_eq!(compute("/src/bin/test.txt"), 6440);
    }

    #[test]
    fn test_point_value() {
        let g = Game{
            hand: "AA3AA".into(),
            bid: 0
        };
        assert_eq!(g.value(), 9);
    }
}
