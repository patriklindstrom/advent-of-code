use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let path = "testdata/almanac.txt";
    let mut lines = read_lines(path)?;

    // Parse the initial seeds
    let seeds_line = lines.next().unwrap()?;
    let seeds: Vec<i32> = seeds_line.split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    // Skip the blank line and start parsing maps
    lines.next();

    let mut maps = HashMap::new();
    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() { continue; }

        let parts: Vec<&str> = line.split(": ").collect();
        let map_name = parts[0].to_string();  // Clone the string
        let mut map = Vec::new();

        while let Some(Ok(line)) = lines.next() {
            if line.is_empty() { break; }

            let nums: Vec<i32> = line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            map.push((nums[0], nums[1], nums[2]));
        }

        maps.insert(map_name, map);  // No longer borrows from `line`
    }

    // Process each seed through the maps
    let mut lowest_location = i32::MAX;
    for seed in seeds {
        let mut current_value = seed;
        println!("Processing seed: {}", seed);

        for (map_name, map) in &maps {
            current_value = map_value(current_value, map);
            println!("{} -> {}: {}", map_name, map_name.split("-").last().unwrap(), current_value);
        }

        if current_value < lowest_location {
            lowest_location = current_value;
        }
    }

    println!("Lowest location number: {}", lowest_location);
    Ok(())
}

fn map_value(value: i32, map: &Vec<(i32, i32, i32)>) -> i32 {
    for &(dest_start, src_start, length) in map {
        if value >= src_start && value < src_start + length {
            return dest_start + (value - src_start);
        }
    }
    value
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
