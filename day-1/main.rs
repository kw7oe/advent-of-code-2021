use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = File::open("sample.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let numbers: Vec<&str> = contents.trim().split("\n").collect();

    let mut i = 0;
    let mut count = 0;

    loop {
        if i == numbers.len() - 1 {
            break;
        }

        if numbers[i + 1] > numbers[i] {
            count += 1;
        }

        i += 1;
    }

    println!("Count: {}", count);

    Ok(())
}
