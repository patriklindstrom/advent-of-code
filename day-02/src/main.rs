use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("testdata/testgame.csv");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let max_cubes   = [("red", 12), ("green", 13), ("blue", 14)]
        .iter().cloned().collect::<HashMap<_, _>>();

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        let parts: Vec<&str> = line.split(':').collect();
        let game_id: i32 = parts[0][5..].trim().parse().unwrap();

        let rounds: Vec<&str> = parts[1].split(';').collect();
        let mut game_cubes = HashMap::new();

        for round in rounds {
            let cubes: Vec<&str> = round.split(',').collect();
            for cube in cubes {
                let cube_parts: Vec<&str> = cube.trim().split_whitespace().collect();
                let count: i32 = cube_parts[0].parse().unwrap();
                let color = cube_parts[1];

                *game_cubes.entry(color).or_insert(0) += count;
            }
        }

        println!("Game ID: {}", game_id);
        for (color, count) in game_cubes {
            println!("Color: {}, Count: {}", color, count);
            if count > *max_cubes.get(color).unwrap() {
                println!("The game {} is not plausible", game_id);
            }
        }
    }

    Ok(())
}