use std::collections::HashMap;

fn main() {
    let crabs: Vec<i32> = include_str!("input.txt")
        .trim()
        .split(",")
        .map(|c| c.parse().unwrap())
        .collect();

    let max = crabs.iter().max().unwrap();
    let mut pos_fuels = HashMap::<i32, i32>::new();

    for i in 0..=*max {
        let sum = crabs.iter().fold(0, |acc, c| acc + (c - i).abs());
        pos_fuels.insert(i, sum);
    }

    println!("{:?}", pos_fuels.values().min());
}
