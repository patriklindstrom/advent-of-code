use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buffer = io::BufReader::new(file);
    buffer.lines().collect()
}

fn find_first_and_last_digit(s: &str) -> Vec<char> {
    let map = number_word_map();
    let mut result = Vec::new();

    // Find first valid digit or digit word
    let mut first_digit_found = false;
    let mut current_index = 0;
    while current_index < s.len() && !first_digit_found {
        for end in (current_index + 1)..=s.len() {
            let substring = &s[current_index..end];
            if let Some(&digit) = map.get(substring) {
                result.push(digit);
                first_digit_found = true;
                break;
            }
        }
        if !first_digit_found {
            current_index += 1;
        }
    }

    // Find last valid digit or digit word
    let mut last_digit_found = false;
    let mut current_index = s.len();
    while current_index > 0 && !last_digit_found {
        for start in (0..current_index).rev() {
            let substring = &s[start..current_index];
            if let Some(&digit) = map.get(substring) {
                result.push(digit);
                last_digit_found = true;
                break;
            }
        }
        if !last_digit_found {
            current_index -= 1;
        }
    }

    result
}

fn concatenate_first_last(digits: Vec<char>) -> Option<i32> {
    match digits.first().zip(digits.last()) {
        Some((&first, &last)) => {
            let num_str = format!("{}{}", first, last);
            num_str.parse::<i32>().ok()
        },
        None => None,
    }
}
// To simply the code, we use a HashMap to map the number words to the digits. Both the digits
//  and the number words are mapped to the same character, so we can use the same HashMap.
fn number_word_map() -> HashMap<&'static str, char> {
    let mapping = [
        ("0", '0'), ("zero", '0'),
        ("1", '1'), ("one", '1'),
        ("2", '2'), ("two", '2'),
        ("3", '3'), ("three", '3'),
        ("4", '4'), ("four", '4'),
        ("5", '5'), ("five", '5'),
        ("6", '6'), ("six", '6'),
        ("7", '7'), ("seven", '7'),
        ("8", '8'), ("eight", '8'),
        ("9", '9'), ("nine", '9'),
    ];
    mapping.iter().cloned().collect()
}
fn main() {
     let file_path = "puzzledata/calibrationdata.txt";
     // let file_path = "puzzledata/testdata1.txt";
     // let file_path = "puzzledata/testdata2.txt";
    match read_lines(file_path) {
        Ok(lines) => {
            let mut sum = 0;
            // 'lines' is a Vec<String> containing all data records  in the file
            let no_of_records = lines.len();
            println!("Number of no_of_records in file: {}", no_of_records);

            for cal_val in lines {
                 print!("{};", cal_val);
                let digits = find_first_and_last_digit(&*cal_val);
                 print!("Digits found: {:?};", digits);
                if let Some(number) = concatenate_first_last(digits) {
                    println!(" Number: {}",  number);
                    sum += number;
                } else {
                    println!("***  String: '{}', No digits found", cal_val);
                }
            }
        println!("Total sum of calibration Values: {}", sum);
        }
        Err(e) => {
            println!("Failed to read from file: {}", e);
        }

    }
}



