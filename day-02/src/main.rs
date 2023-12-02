use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("puzzledata/gamedata.csv");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let max_cubes = [("red", 12), ("green", 13), ("blue", 14)]
        .iter().cloned().collect::<HashMap<_, _>>();

    let mut sum_game_id = 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        let parts: Vec<&str> = line.split(':').collect();
        let game_id: i32 = parts[0][5..].trim().parse().unwrap();

        let rounds: Vec<&str> = parts[1].split(';').collect();
        let mut is_plausible = true;

        for round in rounds {
            let cubes: Vec<&str> = round.split(',').collect();
            for cube in cubes {
                let cube_parts: Vec<&str> = cube.trim().split_whitespace().collect();
                let count: i32 = cube_parts[0].parse().unwrap();
                let color = cube_parts[1];

                if count > *max_cubes.get(color).unwrap() {
                    is_plausible = false;
                    break;
                }
            }
            if !is_plausible {
                break;
            }
        }

        if is_plausible {
            sum_game_id += game_id;
        }
    }

    println!("Sum of Game IDs for plausible games: {}", sum_game_id);

    Ok(())
}