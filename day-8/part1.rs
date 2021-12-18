fn main() {
    let total = include_str!("input.txt").lines().fold(0, |acc, string| {
        let (_, output) = string.split_once('|').unwrap();

        let increment = output.split(" ").fold(0, |sum, word| {
            if [2, 3, 4, 7].contains(&word.len()) {
                sum + 1
            } else {
                sum
            }
        });

        acc + increment
    });

    println!("Total: {}", total);
}
