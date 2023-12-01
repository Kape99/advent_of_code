use std::fs::read_to_string;


fn main() {
    let lines = read_lines("src/input.txt");

    println!("The final calibration value is: {}",compute_calibrations(lines));

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