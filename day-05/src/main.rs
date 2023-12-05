use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let path = "puzzledata/input.txt";
    let mut lines = read_lines(path)?;

    // Parse the initial seeds
    let seeds_line = lines.next().unwrap()?;
    let seeds: Vec<i64> = seeds_line.split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    // Skip the blank line and start parsing maps
    lines.next();

    let mut maps = HashMap::new();
    let map_order = vec![
        "seed-to-soil map", "soil-to-fertilizer map", "fertilizer-to-water map",
        "water-to-light map", "light-to-temperature map", "temperature-to-humidity map",
        "humidity-to-location map"
    ];

    for map_name in map_order.iter() {
        lines.next(); // Skip the map name line
        let mut map = Vec::new();

        while let Some(Ok(line)) = lines.next() {
            if line.is_empty() { break; }

            let nums: Vec<i64> = line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            map.push((nums[0], nums[1], nums[2]));
        }

        maps.insert(*map_name, map);
    }

    // Process each seed through the maps in the correct order
    let mut lowest_location = i64::MAX;
    for seed in &seeds {
        let mut current_value = *seed;
        println!("Processing seed: {}", seed);

        for map_name in &map_order {
            let map = maps.get(*map_name).unwrap();
            current_value = map_value(current_value, map);
            println!("{}: {}", map_name, current_value);
        }

        if current_value < lowest_location {
            lowest_location = current_value;
        }
    }

    println!("Lowest location number: {}", lowest_location);
    Ok(())
}

fn map_value(value: i64, map: &Vec<(i64, i64, i64)>) -> i64 {
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
