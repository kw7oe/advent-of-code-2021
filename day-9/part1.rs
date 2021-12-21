fn main() {
    let input = include_str!("input.txt");
    let numbers: Vec<u8> = input
        .lines()
        .flat_map(|l| {
            l.split("")
                .filter(|c| !c.is_empty())
                .map(|c| c.parse().unwrap())
        })
        .collect();
    let col_len = input.lines().next().unwrap().len();

    let mut sum: u16 = 0;
    for i in 0..numbers.len() {
        let me = numbers[i];
        let mut vec = Vec::new();

        if i + 1 < numbers.len() {
            vec.push(numbers[i + 1]);
        }

        if i != 0 && i % col_len != 0 {
            vec.push(numbers[i - 1]);
        }

        if i > col_len - 1 {
            vec.push(numbers[i - col_len]);
        }

        if i + col_len < numbers.len() {
            vec.push(numbers[i + col_len]);
        }

        if vec.iter().all(|&i| i > me) {
            sum += me as u16 + 1;
        }

        // println!("{}: {:?}", numbers[i], vec);
        // if i == col_len - 1 || (i > col_len && (i + 1 - col_len) % col_len == 0) {
        //     println!("");
        // }
    }

    println!("");
    println!("Total: {}", sum);
}
