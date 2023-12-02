use std::fs::read_to_string;

fn main() {
    let lines = read_lines("src/input.txt");

    let mut calibration_count = 0;

    for line in &lines{
        let new_line = extract_first_and_last(line);

        let new_line_converted = convert_to_digit(new_line);

        calibration_count += compute_calibrations(new_line_converted)
    }

    println!("{calibration_count}");
}

fn extract_first_and_last(line: &String) -> String {
    let values: [&str; 18] = [
        "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7",
        "eight", "8", "nine", "9",
    ];

    let mut first: (&str, Option<usize>) = ("", None);
    let mut last: (&str, Option<usize>) = ("", None);

    for value in values {
        match line.find(value) {
            Some(index) => {
                if first.1.is_none() || index <= first.1.unwrap() {
                    first.0 = value;
                    first.1 = Some(index);
                }
            }
            None => {}
        }
        match line.rfind(value) {
            Some(index) => {
                if last.1.is_none() || index >= last.1.unwrap() {
                    last.0 = value;
                    last.1 = Some(index);
                }
            }
            None => {}
        }
    }

    let mut new_line = first.0.to_owned();
    new_line.push_str(last.0);

    return new_line;
}

fn convert_to_digit(line: String) -> String {
    let conversion_values = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let mut new_line: String = line;
    for conv in &conversion_values {
        new_line = new_line.replace(&conv.0, &conv.1);
    }

    return new_line;
}

fn compute_calibrations(line: String) -> u32 {
    let mut calibration: u32 = 0;
    let mut first_digit: Option<u32> = None;
    let mut last_digit: Option<u32> = None;

    for ch in line.chars() {
        // is a digit in base 10
        match ch.to_digit(10) {
            Some(num) => {
                match first_digit {
                    None => first_digit = Some(num * 10),
                    Some(_) => {}
                }
                last_digit = Some(num);
            }
            None => {}
        }
    }

    // println!( "{} {}{}",line, first_digit.unwrap(),last_digit.unwrap());

    match first_digit {
        Some(num) => calibration += num,
        None => panic!(),
    }
    match last_digit {
        Some(num) => calibration += num,
        None => panic!(),
    }

    return calibration;
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
