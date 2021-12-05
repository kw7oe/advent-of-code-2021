use std::env;
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut args: env::Args = env::args();
    let file_path = args.nth(1).expect("input file path is expected!");

    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let numbers: Vec<u16> = contents
        .trim()
        .split("\n")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut i = 0;
    let mut count = 0;

    loop {
        if i == numbers.len() - 3 {
            break;
        }

        let sum1 = numbers[i] + numbers[i + 1] + numbers[i + 2];
        let sum2 = numbers[i + 1] + numbers[i + 2] + numbers[i + 3];

        if sum1 < sum2 {
            count += 1;
        }

        i += 1;
    }

    println!("Count: {}", count);

    Ok(())
}
