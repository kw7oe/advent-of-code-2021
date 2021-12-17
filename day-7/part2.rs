use std::collections::HashMap;

fn main() {
    let crabs: Vec<i32> = include_str!("input.txt")
        .trim()
        .split(",")
        .map(|c| c.parse().unwrap())
        .collect();

    let max = crabs.iter().max().unwrap();
    let mut pos_fuels = HashMap::<i32, u128>::new();

    for i in 0..=*max {
        let sum = crabs.iter().fold(0, |acc, c| {
            let n: u128 = (c - i).abs() as u128;
            let fuel: u128 = ((n + 1) * n) / 2;
            acc + fuel
        });
        pos_fuels.insert(i, sum);
    }

    // println!("{:?}", pos_fuels);
    println!("{:?}", pos_fuels.values().min());
}
