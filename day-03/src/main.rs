use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("puzzledata/input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut records: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        records.push(line.chars().collect());
    }

    let mut sum = 0;

    for i in 0..records.len() {
        let mut j = 0;
        while j < records[i].len() {
            if records[i][j].is_digit(10) {
                let mut number = String::new();
                let start = j;
                while j < records[i].len() && records[i][j].is_digit(10) {
                    number.push(records[i][j]);
                    j += 1;
                }
                let partnumber: i32 = number.parse().unwrap();

                if is_partnumber(&records, i, start, j - 1) {
                    println!("Partnumber on row {}: {}", i + 1, partnumber);
                    sum += partnumber;
                }
            } else {
                j += 1;
            }
        }
    }

    println!("Sum of partnumbers: {}", sum);

    Ok(())
}

fn is_partnumber(records: &Vec<Vec<char>>, row: usize, start: usize, end: usize) -> bool {
    let is_symbol = |c: char| !c.is_digit(10) && c != '.' && c != ' ';

    for i in row as isize - 1..=row as isize + 1 {
        if i < 0 || i >= records.len() as isize {
            continue;
        }
        for j in start as isize - 1..=end as isize + 1 {
            if j < 0 || j >= records[i as usize].len() as isize {
                continue;
            }
            if is_symbol(records[i as usize][j as usize]) {
                return true;
            }
        }
    }

    false
}
