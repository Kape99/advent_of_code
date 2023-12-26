use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::Chars;


struct Place{
    name: String
}

struct Direction{
    left: String,
    right: String
}

fn main() {
    let mut lines = read_lines("src/bin/input.txt");
    println!("{}", lines.len());

    let instruction = lines.remove(0);
    lines.remove(0);

    let mut inst: HashMap<String, Direction> = HashMap::new();

    for line in lines{

        let new  = line.replace(' ', "").replace('(',"").replace(')',"");
        let key = new.split_at(line.find('=').expect("the = should exist"));
        let (left, right) =  key.1.split_at(key.1.find(',').expect("The , should exist"));
        let ke:String = key.0.replace("=","").into();
        let dir = Direction{
            left: left.replace(",","").to_string(),
            right: right.replace(",","").to_string()
        };
        inst.insert(ke, dir);
    }

    for i in &inst{
        print!("{} ", i.0);
        print!("{} ", i.1.left);
        println!("{}", i.1.right)

    }

    let mut current:String = "AAA".to_string();
    let mut index = 0;
    let mut counter = 0;

    while &current != "ZZZ" {
        let dir = inst.get(&current).expect("the direction should exist");
        match instruction.chars().collect::<Vec<char>>()[index] {
            'L' => current = dir.left.to_owned(),
            'R' => current = dir.right.to_owned(),
            _ => panic!()
        }
        println!("{counter} - {current} - {index} - {} - {}",instruction.len(), instruction.chars().collect::<Vec<char>>()[index]);
        index += 1;
        index = index % instruction.len();

        counter += 1;

    }

    println!("{counter}")
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
