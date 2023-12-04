use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("testdata/cards.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut total_points = 0;
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(" | ").collect();
        if parts.len() != 2 {
            continue; // Skip lines that don't match the format
        }
        // Split the line into winning numbers and my numbers
        let winning_numbers: Vec<i32> = parts[0].split_whitespace().filter_map(|s| s.parse().ok()).collect();
        let my_numbers: Vec<i32> = parts[1].split_whitespace().filter_map(|s| s.parse().ok()).collect();
        // Calculate and print the points for this card
        let points = calculate_points(&winning_numbers, &my_numbers);
        println!("Card: {}", parts[0]);
        println!("Winning numbers found: {:?}", winning_numbers.iter().filter(|&&num| my_numbers.contains(&num)).collect::<Vec<_>>());
        println!("Points for this card: {}\n", points);
        total_points += points;
    }
    println!("The Elf's pile of scratchcards is worth in total : {} points", total_points);
    Ok(())
}

fn calculate_points(winning_numbers: &[i32], my_numbers: &[i32]) -> i32 {
    let mut points = 0;
    let mut first_match = true;

    for &num in winning_numbers {
        if my_numbers.contains(&num) {
            if first_match {
                points += 1;
                first_match = false;
            } else {
                points *= 2;
            }
        }
    }

    points
}

