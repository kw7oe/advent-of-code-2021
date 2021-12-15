fn main() {
    let mut fishes: Vec<u128> = include_str!("sample.txt")
        .trim()
        .split(",")
        .map(|c| c.parse().unwrap())
        .collect();

    let days = 80;

    println!("Initial state: {:?}", fishes);

    let mut new_fish = 0;
    for i in 1..=days {
        println!("Day {}", i);
        for _ in 0..new_fish {
            fishes.push(9);
            new_fish = 0;
        }

        for f in fishes.iter_mut() {
            if *f == 0 {
                *f = 6;
            } else {
                *f -= 1;
            }

            if *f == 0 {
                new_fish += 1;
            }
        }

        // println!("After {} days: {:?}", i, fishes);
    }

    println!("Total: {}", fishes.len());
}
