const WIDTH: usize = 10;

fn main() {
    let mut numbers: Vec<u8> = include_str!("input.txt")
        .lines()
        .flat_map(|line| {
            line.split("")
                .filter(|c| !c.is_empty())
                .map(|c| c.parse().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect();

    let mut flashes: u16 = 0;

    for i in 1..=100 {
        println!("After step {}:", i);

        for j in 0..numbers.len() {
            numbers[j] += 1;
        }

        for j in 0..numbers.len() {
            if numbers[j] > 9 {
                flashes += 1;
                numbers[j] = 0;
                flash(&mut numbers, j, &mut flashes);
            }
        }

        println!("\nFinal output:");
        print_boards(&numbers);

        println!("");
    }

    println!("Total flashes: {}", flashes);
}

fn print_boards(numbers: &Vec<u8>) {
    for (i, n) in numbers.iter().enumerate() {
        if *n != 0 {
            print!("{}", n);
        } else {
            print!("\x1b[93m0\x1b[0m");
        }

        if i % WIDTH == 9 {
            println!("");
        }
    }
}

fn flash(numbers: &mut Vec<u8>, index: usize, flashes: &mut u16) {
    let mut top = None;
    let mut left = None;
    let mut right = None;
    let mut bottom = None;
    let mut top_left = None;
    let mut top_right = None;
    let mut bottom_left = None;
    let mut bottom_right = None;

    let col = index % WIDTH;
    let row = index / WIDTH;

    match (row, col) {
        (0, 0) => {
            right = Some(index + 1);
            bottom = Some(index + WIDTH);
            bottom_right = Some(bottom.unwrap() + 1);
        }
        (0, 9) => {
            left = Some(index - 1);
            bottom = Some(index + WIDTH);
            bottom_left = Some(bottom.unwrap() - 1);
        }
        (9, 0) => {
            right = Some(index + 1);
            top = Some(index - WIDTH);
            top_right = Some(top.unwrap() + 1);
        }
        (9, 9) => {
            left = Some(index - 1);
            top = Some(index - WIDTH);
            top_left = Some(top.unwrap() - 1);
        }
        (_, 0) => {
            top = Some(index - WIDTH);
            top_right = Some(top.unwrap() + 1);

            bottom = Some(index + WIDTH);
            bottom_right = Some(bottom.unwrap() + 1);

            right = Some(index + 1);
        }
        (_, 9) => {
            top = Some(index - WIDTH);
            top_left = Some(top.unwrap() - 1);

            bottom = Some(index + WIDTH);
            bottom_left = Some(bottom.unwrap() - 1);

            left = Some(index - 1);
        }
        (0, _) => {
            bottom = Some(index + WIDTH);
            bottom_right = Some(bottom.unwrap() + 1);
            bottom_left = Some(bottom.unwrap() - 1);

            left = Some(index - 1);
            right = Some(index + 1);
        }
        (9, _) => {
            top = Some(index - WIDTH);
            top_right = Some(top.unwrap() + 1);
            top_left = Some(top.unwrap() - 1);

            left = Some(index - 1);
            right = Some(index + 1);
        }
        _ => {
            top = Some(index - WIDTH);
            top_right = Some(top.unwrap() + 1);
            top_left = Some(top.unwrap() - 1);

            bottom = Some(index + WIDTH);
            bottom_right = Some(bottom.unwrap() + 1);
            bottom_left = Some(bottom.unwrap() - 1);

            left = Some(index - 1);
            right = Some(index + 1);
        }
    }

    [
        top,
        top_right,
        top_left,
        bottom,
        bottom_right,
        bottom_left,
        left,
        right,
    ]
    .iter()
    .for_each(|index| {
        if let Some(w) = index {
            let number = numbers[*w] + 1;

            if number > 9 {
                *flashes += 1;
                numbers[*w] = 0;
                flash(numbers, *w, flashes);
            } else if number != 1 {
                numbers[*w] += 1;
            }
        }
    });
}
