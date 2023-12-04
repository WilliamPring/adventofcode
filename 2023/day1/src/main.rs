// use std::fs::File;
// use std::io::{self, BufRead};
// use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn calculate_calibration_value(line: &str) -> u32 {
    let mut acc = String::from("");
    let mut second_char: Option<char> = None;
    let mut isFirst = false;
    for c in line.chars() {
        if c.is_ascii_digit() && isFirst == false {
            isFirst = true;
            acc = c.to_string();
        } else if(c.is_ascii_digit()) {
            second_char = Some(c);
        }
    }
    if acc.len()==1 && second_char == None {
        acc = acc.clone() + &acc;
    } else {
        acc = acc.clone() +  &second_char.map(|c| c.to_string()).unwrap_or_default();
    }
    let data = acc.parse::<u32>().unwrap();
    data
}


fn main() {
    let file = File::open("input").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut total_calibration_value = 0;
    for line in reader.lines() {
        if let Ok(line) = line {
            let calibration_value = calculate_calibration_value(&line);
            total_calibration_value += calibration_value;
            println!("Line: {}, Calibration Value: {}", line, calibration_value);
        }
    }
    println!("{}", total_calibration_value.to_string());
    // File hosts.txt must exist in the current path
    // if let Ok(lines) = read_lines("./input") {
    //     for line in lines {
    //         let calibration_value = calculate_calibration_value(line);
    //         total_calibration_value += calibration_value;
    //         println!("Line: {}, Calibration Value: {}", line, calibration_value);
    //     }
        // Consumes the iterator, returns an (Optional) String
        // for line in lines {
        //     if let Ok(data_line) = line {
        //         let mut left_pointer: i32  = 0;
        //         let mut rightPointer = data_line.len();
        //         let mut left_value = 0;
        //         while (left_pointer <= rightPointer) {
        //             let left_data = data_line.chars().nth(left_pointer);
        //             if let Some(temp_left_data) = data_line.chars().nth(left_pointer) {
        //                 if(temp_left_data.is_numeric()) {
        //                     left_value = temp_left_data.();
        //                 } else {
        //                     left_pointer+=1;
        //                 }
        //             }

        //             // if let Some(db_path) 
        //             // if(left_data.is_numeric()) {
                        
        //             // }
        //         }


        //         println!("{leftPointer}, {rightPointer}");
        //     }
        // }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// where P: AsRef<Path>, {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }