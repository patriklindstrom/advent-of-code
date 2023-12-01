use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buffer = io::BufReader::new(file);
    buffer.lines().collect()
}
fn find_digits(s: &str) -> Vec<char> {
    s.chars().filter(|c| c.is_digit(10)).collect()
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
fn main() {
    let file_path = "puzzledata/calibrationdata.txt";
    match read_lines(file_path) {
        Ok(lines) => {
            let mut sum = 0;
            // 'lines' is a Vec<String> containing all data records  in the file
            let no_of_records = lines.len();
            println!("Number of no_of_records in file: {}", no_of_records);

            for cal_val in lines {
                // println!("{}", cal_val);
                let digits = find_digits(&*cal_val);
                // println!("Digits found: {:?}", digits);
                if let Some(number) = concatenate_first_last(digits) {
                   // println!("String: '{}', Number: {}", cal_val, number);
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



