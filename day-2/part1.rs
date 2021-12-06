use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

enum Direction {
    Forward,
    Up,
    Down,
}

use Direction::*;

impl From<&str> for Direction {
    fn from(dir: &str) -> Self {
        match dir {
            "forward" => Forward,
            "up" => Up,
            "down" => Down,
            _ => panic!("unexpected direction!"),
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut args: env::Args = env::args();
    let file_path = args.nth(1).expect("input file is expected");
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let (depth, horizontal_position) = reader.lines().fold((0, 0), |acc, entry| {
        let (direction, step) = parse_entry(entry.unwrap());
        let (depth, hpos) = acc;

        match direction {
            Forward => (depth, hpos + step),
            Up => (depth - step, hpos),
            Down => (depth + step, hpos),
        }
    });

    println!(
        "Depth: {}, Horizontal Position: {}",
        depth, horizontal_position
    );

    println!("Answer: {}", depth * horizontal_position);

    Ok(())
}

fn parse_entry(entry: String) -> (Direction, u32) {
    match entry.split(" ").collect::<Vec<&str>>()[..] {
        [direction, step_value] => {
            let step: u32 = step_value.parse().unwrap();
            (Direction::from(direction), step)
        }
        _ => panic!("unexpected entry format!"),
    }
}
