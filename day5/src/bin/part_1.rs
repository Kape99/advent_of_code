use std::{fs::read_to_string, result, collections::HashSet, cmp::min};

#[derive(Debug)]
struct ConversionField{
    source_start: i64,
    destination_index: i64,
    size: i64,
}

impl ConversionField{
    fn try_convertion(&self, value: i64) -> Option<i64>{
        let validity_range = self.source_start..self.source_start+self.size;
        if validity_range.contains(&value){
            return Some(value + (self.destination_index-self.source_start))
        }
        None
    }
}


#[derive(Debug)]
struct ConversionMap{
    conversion_fields: Vec<ConversionField>
}

impl ConversionMap{
    fn convert(&self, value: i64) -> i64{
        for field in &self.conversion_fields{
            if let Some(result) = field.try_convertion(value) {
                return result;
            }
        }
        value
    }
} 


fn main() {
    let lines = read_lines("src/bin/input.txt");

    let seeds: Vec<i64> = lines[0][7..].split_whitespace().map(|s| s.parse::<i64>().expect("number!!!")).collect();

    let mut skip_line = false;

    let mut conv_fields: HashSet<ConversionField> = HashSet::new();
    
    let mut conv_list: Vec<ConversionMap> = vec![];
    
    for line in &lines[1..]{
        if skip_line{
            skip_line = false;
            
            let new_map: ConversionMap = ConversionMap { conversion_fields: vec![] };
            conv_list.push(new_map);
            continue;
        }
        
        if line == ""{
            skip_line = true;
            conv_fields.clear();
            continue;
        }

        conv_list.last_mut().expect("there must be an element").conversion_fields
            .push({
                let list: Vec<_>= line.split_whitespace().map(|f| f.parse::<i64>().expect("should be a number")).collect();
                let source_start: i64= list[1];
                let destination_index: i64= list[0];
                let size: i64= list[2];
                ConversionField{
                    source_start,destination_index,size}
            })
    }

    dbg!(&conv_list);

    let mut lowest: Option<i64> = None;

    for mut seed in seeds{
        let mut new = seed;
        for con in &conv_list{
            new = con.convert(new);
            println!("{new}");
        } 
        println!("");
        if None == lowest { lowest = Some(new)}
        else {
            lowest = Some(min(lowest.unwrap(), new))
            
        }
    }
    println!("Lowest {}", lowest.unwrap());

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
