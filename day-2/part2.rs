use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file_path = env::args().nth(1).expect("input file is expected");
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let (depth, hpos, _) = reader
        .lines()
        .map(|l| {
            let s = l.unwrap();
            let (x, y) = s.split_once(" ").unwrap();
            (x.to_owned(), y.to_owned())
        })
        .fold((0, 0, 0), |(depth, hpos, aim), (direction, step)| {
            match (direction.as_str(), step.parse::<u32>().unwrap()) {
                ("forward", s) => (depth + (s * aim), hpos + s, aim),
                ("down", s) => (depth, hpos, aim + s),
                ("up", s) => (depth, hpos, aim - s),
                _ => unreachable!(),
            }
        });

    println!("Depth: {}, H Pos: {}", depth, hpos);
    println!("Answer: {}", depth * hpos);

    Ok(())
}
