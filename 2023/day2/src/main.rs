use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut acc = 0;
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./data") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line_result) = line {
                let line_result_parts: Vec<&str> = line_result.split(':').collect();

                if let Some(day) = extract_day(&line_result_parts) {
                    println!("Day: {}", day);
                    if let Some(second_element) = line_result_parts.get(1) {
                        println!("Second element: {}", second_element.trim());
                        let color_maps = parse_data(second_element.to_string());
                        if(color_maps==true) {
                            acc = acc + day;
                        }
                    } else {
                        println!("Second element not found.");
                    }
                } else {
                    println!("Unable to extract day. {}",  acc.to_string());
                }
            }
        }
        println!("inside {}", acc.to_string());
    }
}
fn extract_game_number(game_string: &str) -> Option<u32> {
    let parts: Vec<&str> = game_string.split(' ').collect();
    let game_number_str = parts.get(1)?;
    game_number_str.parse::<u32>().ok()
}

fn extract_day(line_result_parts: &[&str]) -> Option<u32> {
    if let Some(first_element) = line_result_parts.get(0) {
        extract_game_number(first_element)
    } else {
        None
    }
}

fn parse_data(data:String) -> bool {
    // let mut counts: HashMap<_, _> = HashMap::new();
    let mut red: bool = false;
    let mut green: bool = false;
    let mut blue: bool = false;

    for section in data.split(';') {
        for color_count in section.split(',') {
            let parts: Vec<&str> = color_count.trim().split_whitespace().collect();
            if let Some(color) = parts.get(1) {
                if let Some(value) = parts.get(0) {
                    let my_int = value.parse::<i32>().unwrap();
                    if(*color == "red") {
                        if my_int <= 12 {
                            red = true;

                        } else {
                            return false
                        }
                    }
                    if(*color == "green") {
                        if my_int <= 13 {
                            green = true;

                        } else {
                            return false

                        }
                    }
                    if(*color == "blue") {
                        if my_int <= 14 {
                            blue = true;
                        } else {
                            return false

                        }
                    }
                }
            }
        }
    }
    red && blue && green
}
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}