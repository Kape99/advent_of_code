use std::fs::read_to_string;


fn main() {
    let lines = read_lines("src/input.txt");

    let test = "xjvbkbtdqhgvsseven719fiveseven";

    
    println!("The final calibration value is: {}", compute_calibrations_correct(lines));
    println!("{}", test.replace("seven", "7"));

}

fn compute_calibrations_correct(lines: Vec<String>) -> u32{
    let conversion_values = [
        ("one","1"),
        ("two","2"),
        ("three","3"),
        ("four","4"),
        ("five","5"),
        ("six","6"),
        ("seven","7"),
        ("eight","8"),
        ("nine","9"),
        ];

    // nice aproach but "zoneight234" ==> "z1ight234" the eight cannot be completed....
    let mut new_lines: Vec<String> = vec![];
    for line in &lines{
        let mut new_line: String = line.to_string();
        for conv in &conversion_values{
            new_line = new_line.replace(&conv.0, &conv.1);
        }
        new_lines.push(new_line);
        // println!("{} : {}", line, new_lines.last().unwrap());
    }

    return compute_calibrations(new_lines);
}

fn compute_calibrations(lines: Vec<String>) -> u32{
    let mut calibration_sum:u32 = 0;
    for line in lines{
        let mut first_digit : Option<u32> = None;
        let mut last_digit : Option<u32> = None;

        for ch in line.chars(){
            // is a digit in base 10 
            match ch.to_digit(10) {
                Some(num) => {
                    match first_digit {
                        None => first_digit = Some(num * 10),
                        Some(_)=>{}, 
                    }
                    last_digit = Some(num);
                },
                None=>{}
            }
        }

        // println!( "{} {}{}",line, first_digit.unwrap(),last_digit.unwrap());

        match first_digit{
            Some(num) => calibration_sum += num,
            None => panic!(),
        }        
        match last_digit{
            Some(num) => calibration_sum += num,
            None => panic!(),
        }

    }
    return calibration_sum;
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    match read_to_string(filename){
        Ok(lines) => {
            for line in lines.lines(){
                result.push(line.to_string());
            }
        },
        Err(msg) => println!("ERROR: {msg}"),
    }
    result
}