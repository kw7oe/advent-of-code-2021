use std::collections::HashMap;

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

    let mut graphs: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut lowest_points: Vec<usize> = Vec::new();

    for i in 0..numbers.len() {
        let me = numbers[i];
        let mut vec = Vec::new();
        let mut indexes: Vec<usize> = Vec::new();

        if (i + 1) % col_len != 0 && i + 1 < numbers.len() {
            let index = i + 1;
            vec.push(numbers[index]);
            indexes.push(index);
        }

        if i != 0 && i % col_len != 0 {
            let index = i - 1;

            vec.push(numbers[index]);
            indexes.push(index);
        }

        if i > col_len - 1 {
            let index = i - col_len;

            vec.push(numbers[index]);
            indexes.push(index);
        }

        if i + col_len < numbers.len() {
            let index = i + col_len;

            vec.push(numbers[index]);
            indexes.push(index);
        }

        if vec.iter().all(|&i| i > me) {
            lowest_points.push(i);
        }

        if me != 9 {
            graphs.insert(i, indexes);
        }

        // println!("{}: {:?}", numbers[i], vec);
        // if i == col_len - 1 || (i > col_len && (i + 1 - col_len) % col_len == 0) {
        //     println!("");
        // }
    }

    let mut basins: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in lowest_points {
        let mut vec = Vec::new();
        vec.push(i);

        let mut surrounding = graphs.get(&i).unwrap().clone();
        loop {
            let mut new_surr: Vec<usize> =
                surrounding.clone().iter().fold(Vec::new(), |mut acc, s| {
                    if let Some(surr) = graphs.get(s) {
                        vec.push(*s);
                        for s in surr {
                            if !vec.contains(s) {
                                acc.push(*s);
                            }
                        }
                    }

                    acc
                });

            new_surr.sort();
            new_surr.dedup();

            // println!("{:?}: {:?}", &surrounding, &new_surr);
            if new_surr.is_empty() {
                break;
            }

            surrounding.clear();
            for s in &new_surr {
                surrounding.push(*s);
            }
        }

        basins.insert(i, vec);
        // println!("{}: \nBasins: {:?}\n", i, basins.get(&i).unwrap());
    }

    let mut vec: Vec<usize> = basins.values().map(|c| c.len()).collect();
    vec.sort();

    let mut total = 1;
    for _ in 0..3 {
        total *= vec.pop().unwrap();
    }

    println!("Total: {}", total);
}
