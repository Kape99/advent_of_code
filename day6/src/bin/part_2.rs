use std::fs::read_to_string;

fn main() {
    let lines = read_lines("src/bin/input.txt");

    let times: Vec<u64> = lines[0]
        .replace(' ', "")
        .split_at(1 + lines[0].find(':').expect(": should exist"))
        .1
        .split_whitespace()
        .map(|time| time.parse::<u64>().expect("should be a number"))
        .collect();
    let distances: Vec<u64> = lines[1]
        .replace(' ', "")
        .split_at(1 + lines[1].find(':').expect(": should exist"))
        .1
        .split_whitespace()
        .map(|time| time.parse::<u64>().expect("should be a number"))
        .collect();

    let mut result: u64 = 1;

    for (t, _) in times.iter().enumerate() {
        let mut winnings = 0;
        let time = times[t];

        for charge in 0..time + 1 {
            let distance = charge * (time - charge);
            if distance > distances[t] {
                winnings += 1;
            }
        }

        result *= winnings;
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

#[cfg(test)]
mod test {
    use super::*;
}
